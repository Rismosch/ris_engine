use std::sync::Arc;

use sdl2_sys::SDL_WindowFlags;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::image::view::ImageView;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::SwapchainPresentInfo;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;

use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::logic_data::LogicData;
use ris_error::RisResult;
use ris_math::matrix4x4::Matrix4x4;
use ris_video::imgui::RisImgui;
use ris_video::vulkan::gpu_objects::UniformBufferObject;
use ris_video::vulkan::renderer::Fence;
use ris_video::vulkan::renderer::Renderer;

pub struct OutputFrame {
    renderer: Renderer,
    recreate_swapchain: bool,
    fences: Vec<Option<Arc<Fence>>>,
    previous_image: usize,
    imgui: RisImgui,
}

impl OutputFrame {
    pub fn new(renderer: Renderer, imgui: RisImgui) -> Self {
        let frames_in_flight = renderer.get_image_count();
        let fences: Vec<Option<Arc<Fence>>> = vec![None; frames_in_flight];

        Self {
            renderer,
            recreate_swapchain: false,
            fences,
            previous_image: 0,
            imgui,
        }
    }

    pub fn run(&mut self, logic: &LogicData, frame: Frame) -> RisResult<()> {
        let (recreate_viewport, reload_shaders) = if logic.reload_shaders {
            (true, true)
        } else if logic.window_size_changed.is_some() {
            (true, false)
        } else {
            (false, false)
        };

        let window_flags = self.renderer.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;
        if is_minimized {
            return Ok(());
        }

        if recreate_viewport {
            if reload_shaders {
                self.renderer.reload_shaders()?;
            }

            self.renderer.recreate_swapchain()?;
            self.recreate_swapchain = false;
        }

        let ui = self
            .imgui
            .backend
            .prepare_frame(logic, frame, &self.renderer);
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

        if let Some(image_fence) = &self.fences[image as usize] {
            ris_error::unroll!(image_fence.wait(None), "failed to wait on fence")?;
        }

        // logic that uses the GPU resources that are currently notused (have been waited upon)
        let scene = &logic.scene;
        let view = Matrix4x4::view(scene.camera_position, scene.camera_rotation);

        let fovy = 60. * ris_math::DEG2RAD;
        let (w, h) = self.renderer.window.vulkan_drawable_size();
        let aspect_ratio = w as f32 / h as f32;
        let near = 0.01;
        let far = 0.1;
        let proj = Matrix4x4::perspective_projection(fovy, aspect_ratio, near, far);

        let view_proj = proj * view;

        let ubo = UniformBufferObject {
            view,
            proj,
            view_proj,
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
        let previous_future = match self.fences[self.previous_image].clone() {
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
            Ok(fence) => Some(Arc::new(fence)),
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                None
            }
            Err(e) => {
                ris_log::warning!("failed to flush future: {}", e);
                None
            }
        };

        self.previous_image = image as usize;

        Ok(())
    }
}
