use std::sync::Arc;

use sdl2::video::Window;
use sdl2_sys::SDL_WindowFlags;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBufferExecFuture;
use vulkano::image::view::ImageView;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::PresentFuture;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::swapchain::SwapchainPresentInfo;
use vulkano::sync::future::FenceSignalFuture;
use vulkano::sync::future::JoinFuture;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;

use ris_data::gameloop::frame::Frame;
use ris_data::god_state::GodState;
use ris_data::god_state::WindowEvent;
use ris_error::RisResult;
use ris_math::space::Space;
use ris_video::imgui::RisImgui;
use ris_video::vulkan::gpu_objects::UniformBufferObject;
use ris_video::vulkan::renderer::Renderer;

type Fence = FenceSignalFuture<
    PresentFuture<
        CommandBufferExecFuture<
            CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>,
        >,
    >,
>;

pub struct OutputFrame {
    recreate_swapchain: bool,
    previous_fence: usize,
    fences: Vec<Option<Arc<Fence>>>,
    imgui: RisImgui,

    // mut be dropped last
    renderer: Renderer,
    window: Window,
}

impl OutputFrame {
    pub fn new(window: Window, renderer: Renderer, imgui: RisImgui) -> RisResult<Self> {
        let frames_in_flight = renderer.get_image_count();
        let mut fences = Vec::with_capacity(frames_in_flight);
        for _ in 0..frames_in_flight {
            fences.push(None);
        }

        Ok(Self {
            recreate_swapchain: false,
            previous_fence: 0,
            fences,
            imgui,
            renderer,
            window,
        })
    }

    pub fn run(&mut self, frame: Frame, state: Arc<GodState>) -> RisResult<()> {
        let window_flags = self.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;
        if is_minimized {
            return Ok(());
        }

        let (recreate_viewport, reload_shaders) = if state.back().reload_shaders {
            (true, true)
        } else {
            match state.back().window_event {
                WindowEvent::SizeChanged(..) => (true, false),
                WindowEvent::None => (false, false),
            }
        };

        let window_size = self.window.size();
        let window_drawable_size = self.window.vulkan_drawable_size();

        if recreate_viewport {
            if reload_shaders {
                self.renderer.reload_shaders()?;
            }

            self.renderer.recreate_viewport(window_drawable_size)?;
        }

        if self.recreate_swapchain {
            self.recreate_swapchain = false;
            self.renderer.recreate_swapchain(window_drawable_size)?;
        }

        let ui = self.imgui.backend.prepare_frame(
            frame,
            state.clone(),
            (window_size.0 as f32, window_size.1 as f32),
            (window_drawable_size.0 as f32, window_drawable_size.1 as f32),
        );
        ui.show_demo_window(&mut true);

        let (image, suboptimal, acquire_future) = match self.renderer.acquire_swapchain_image() {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                self.recreate_swapchain = true;
                return Ok(());
            }
            Err(e) => return ris_error::new_result!("failed to acquire next image: {}", e),
        };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        if let Some(fence) = &self.fences[image as usize] {
            ris_error::unroll!(fence.wait(None), "failed to wait on fence")?;
        }

        // logic that uses the GPU resources that are currently notused (have been waited upon)
        let view = Space::view(state.back().camera_position, state.back().camera_rotation);

        let fovy = ris_math::radians(60.);
        let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
        let aspect_ratio = w / h;
        let near = 0.01;
        let far = 0.1;
        let proj = Space::proj(fovy, aspect_ratio, near, far);

        let proj_view = proj * view;

        let ubo = UniformBufferObject {
            view,
            proj,
            proj_view,
        };
        self.renderer.update_uniform(image as usize, &ubo)?;

        let swapchain_image = &self.renderer.images[image as usize];
        let imgui_target = ris_error::unroll!(
            ImageView::new_default(swapchain_image.clone()),
            "failed to create image view",
        )?;
        let draw_data = self.imgui.backend.context().render();
        let mut imgui_command_buffer_builder = ris_error::unroll!(
            AutoCommandBufferBuilder::primary(
                &self.renderer.allocators.command_buffer,
                self.renderer.queue.queue_family_index(),
                vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
            ),
            "failed to create auto command buffer builder",
        )?;
        self.imgui.renderer.draw(
            imgui_target,
            &mut imgui_command_buffer_builder,
            &self.renderer.allocators,
            draw_data,
        )?;
        let imgui_command_buffer = ris_error::unroll!(
            imgui_command_buffer_builder.build(),
            "failed to build auto command buffer builder",
        )?;

        let use_gpu_resources = false;
        let previous_future = match self.fences[self.previous_fence].clone() {
            None => self.renderer.synchronize().boxed(),
            Some(fence) => {
                if use_gpu_resources {
                    ris_error::unroll!(fence.wait(None), "failed to wait on fence")?;
                }

                fence.boxed()
            }
        };

        if use_gpu_resources {
            // logic that can use every GPU resource (the GPU is sleeping)
        }

        let fence = previous_future
            .join(acquire_future)
            .then_execute(
                self.renderer.queue.clone(),
                self.renderer.command_buffers[image as usize].clone(),
            )
            .map_err(|e| ris_error::new!("failed to execute command buffer: {}", e))?
            .then_execute(self.renderer.queue.clone(), imgui_command_buffer)
            .map_err(|e| ris_error::new!("failed to execute command buffer: {}", e))?
            .then_swapchain_present(
                self.renderer.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.renderer.swapchain.clone(), image),
            )
            .then_signal_fence_and_flush();

        self.fences[image as usize] = match fence {
            Ok(fence) => {
                #[allow(clippy::arc_with_non_send_sync)]
                // false positive, since `FenceSignalFuture` indeed implements `Send`:
                // doc/vulkano/sync/future/struct.FenceSignalFuture.html#impl-Send-for-FenceSignalFuture%3CF%3E
                Some(Arc::new(fence))
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                None
            }
            Err(e) => {
                ris_log::warning!("failed to flush future: {}", e);
                None
            }
        };

        self.previous_fence = image as usize;

        Ok(())
    }
}
