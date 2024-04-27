use std::ptr;

use ash::vk;
use sdl2::video::Window;
use sdl2_sys::SDL_WindowFlags;
use vulkano::command_buffer::CommandBufferExecFuture;
use vulkano::swapchain::PresentFuture;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::sync::future::FenceSignalFuture;
use vulkano::sync::future::JoinFuture;
use vulkano::sync::GpuFuture;

use ris_data::gameloop::frame::Frame;
use ris_data::god_state::GodState;
use ris_data::god_state::WindowEvent;
use ris_error::RisResult;
use ris_math::matrix::Mat4;
use ris_video::vulkan::frame_in_flight::FrameInFlight;
use ris_video::vulkan::renderer::Renderer;
use ris_video::vulkan::swapchain_objects::SwapchainObjects;
use ris_video::vulkan::uniform_buffer_object::UniformBufferObject;

use crate::ui_helper::UiHelper;

type Fence = FenceSignalFuture<
    PresentFuture<
        CommandBufferExecFuture<
            CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>,
        >,
    >,
>;

pub struct OutputFrame {
    //recreate_swapchain: bool,

    current_frame: usize,
    //fences: Vec<Option<Arc<Fence>>>,
    //imgui: RisImgui,

    ui_helper: UiHelper,

    // mut be dropped last
    renderer: Renderer,
    window: Window,
}

impl OutputFrame {
    pub fn new(
        window: Window,
        renderer: Renderer,
        //imgui: RisImgui,
        ui_helper: UiHelper,
    ) -> RisResult<Self> {
        //let frames_in_flight = renderer.get_image_count();
        //let mut fences = Vec::with_capacity(frames_in_flight);
        //for _ in 0..frames_in_flight {
        //    fences.push(None);
        //}

        Ok(Self {
            //recreate_swapchain: false,
            current_frame: 0,
            //fences,
            //imgui,
            ui_helper,
            renderer,
            window,
        })
    }

    pub fn run(
        &mut self,
        frame: Frame,
        state: &mut GodState,
    ) -> RisResult<()> {
        let window_flags = self.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;
        if is_minimized {
            return Ok(());
        }

        let Renderer{
            device,
            graphics_queue,
            present_queue,
            swapchain_objects: SwapchainObjects{
                swapchain_loader,
                swapchain,
                swapchain_extent,
                render_pass,
                pipeline_layout,
                graphics_pipeline,
                framebuffers,
                ..
            },
            vertex_buffer,
            index_buffer,
            frames_in_flight,
            ..
        } = &self.renderer;

        let FrameInFlight {
            command_buffer,
            uniform_buffer_mapped,
            descriptor_set,
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fence,
            ..
        } = &frames_in_flight[self.current_frame];
        let next_frame = (self.current_frame + 1) % frames_in_flight.len();
 
        // wait for the previous frame to finish
        let fence = [*in_flight_fence];
        unsafe{device.wait_for_fences(&fence, true, u64::MAX)}?;
        unsafe{device.reset_fences(&fence)}?;

        // acquire an image from the swap chain
        let acquire_image_result = unsafe{swapchain_loader.acquire_next_image(
            *swapchain,
            u64::MAX,
            *image_available_semaphore,
            vk::Fence::null(),
        )};

        let image_index = match acquire_image_result {
            Ok((image_index, _is_sub_optimal)) => image_index,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR => return self.renderer.recreate_swapchain(self.window.vulkan_drawable_size()),
                vk_result => return ris_error::new_result!("failed to acquire chain image: {}", vk_result),
            },
        };

        // update uniform buffer
        let window_drawable_size = self.window.vulkan_drawable_size();
        let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
        state.camera.aspect_ratio = w / h;
        let view = state.camera.view_matrix();
        let proj = state.camera.projection_matrix();

        let ubo = [UniformBufferObject {
            model: Mat4::init(1.0),
            view,
            proj,
        }];

        unsafe{uniform_buffer_mapped.copy_from_nonoverlapping(ubo.as_ptr(), ubo.len())};

        // record a command buffer which draws the scene onto that image
        let command_buffer_reset_flags = vk::CommandBufferResetFlags::empty();
        unsafe{device.reset_command_buffer(*command_buffer, command_buffer_reset_flags)}?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: ptr::null(),
        };

        unsafe{device.begin_command_buffer(*command_buffer, &command_buffer_begin_info)}?;

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 0.0],
            },
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: ptr::null(),
            render_pass: *render_pass,
            framebuffer: framebuffers[image_index as usize],
            render_area: vk::Rect2D{
                offset: vk::Offset2D {x: 0, y: 0},
                extent: *swapchain_extent,
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe{device.cmd_begin_render_pass(*command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE)};
        unsafe{device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, *graphics_pipeline)};

        let vertex_buffers = [vertex_buffer.buffer];
        let offsets = [0_u64];
        unsafe{device.cmd_bind_vertex_buffers(*command_buffer, 0, &vertex_buffers, &offsets)};
        unsafe{device.cmd_bind_index_buffer(*command_buffer, index_buffer.buffer, 0, vk::IndexType::UINT32)};
        let descriptor_sets = [*descriptor_set];
        unsafe{device.cmd_bind_descriptor_sets(
            *command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            *pipeline_layout,
            0,
            &descriptor_sets,
            &[],
        )};

        let index_count = ris_video::vulkan::INDICES.len() as u32;
        unsafe{device.cmd_draw_indexed(*command_buffer, index_count, 1, 0, 0, 0)};
        unsafe{device.cmd_end_render_pass(*command_buffer)};
        unsafe{device.end_command_buffer(*command_buffer)}?;

        // submit the recorded command buffer
        let wait_semaphores = [*image_available_semaphore];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [*command_buffer];
        let signal_semaphores = [*render_finished_semaphore];
        
        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: command_buffers.len() as u32,
            p_command_buffers: command_buffers.as_ptr(),
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        }];

        unsafe{device.queue_submit(
            *graphics_queue,
            &submit_infos,
            *in_flight_fence,
        )}?;

        // present the swap chain image
        let swapchains = [*swapchain];

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: signal_semaphores.len() as u32,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: swapchains.len() as u32,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
        };

        let queue_present_result = unsafe{swapchain_loader.queue_present(*present_queue, &present_info)};
        let window_event = match queue_present_result {
            Ok(_) => {
                state.window_event
            },
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => {
                    ris_log::fatal!("zwei");
                    let (width, height) = self.window.vulkan_drawable_size();
                    WindowEvent::SizeChanged(width, height)
                },
                vk_result => return ris_error::new_result!("failed to present queue: {}", vk_result),
            },
        };

        if let WindowEvent::SizeChanged(width, height) = window_event {
            ris_log::fatal!("two");
            self.renderer.recreate_swapchain((width, height))?;
        }

        self.current_frame = next_frame;

        //let (recreate_viewport, reload_shaders) = if *state.back.reload_shaders.borrow() {
        //    (true, true)
        //} else {
        //    match *state.back.window_event.borrow() {
        //        WindowEvent::SizeChanged(..) => (true, false),
        //        WindowEvent::None => (false, false),
        //    }
        //};

        //let window_size = self.window.size();
        //let window_drawable_size = self.window.vulkan_drawable_size();

        //if recreate_viewport {
        //    if reload_shaders {
        //        self.renderer.reload_shaders()?;
        //    }

        //    self.renderer.recreate_viewport(window_drawable_size)?;
        //}

        //if self.recreate_swapchain {
        //    self.recreate_swapchain = false;
        //    self.renderer.recreate_swapchain(window_drawable_size)?;
        //}

        ////let ui = self.imgui.backend.prepare_frame(
        ////    frame,
        ////    state.clone(),
        ////    (window_size.0 as f32, window_size.1 as f32),
        ////    (window_drawable_size.0 as f32, window_drawable_size.1 as f32),
        ////);
        ////self.ui_helper.draw(UiHelperDrawData {
        ////    ui,
        ////    logic_future: Some(logic_future),
        ////    frame,
        ////    state: state.clone(),
        ////})?;

        //let (image, suboptimal, acquire_future) = match self.renderer.acquire_swapchain_image() {
        //    Ok(r) => r,
        //    Err(AcquireError::OutOfDate) => {
        //        self.recreate_swapchain = true;
        //        return Ok(());
        //    }
        //    Err(e) => return ris_error::new_result!("failed to acquire next image: {}", e),
        //};

        //if suboptimal {
        //    self.recreate_swapchain = true;
        //}

        //if let Some(fence) = &self.fences[image as usize] {
        //    fence.wait(None)?;
        //}

        //// logic that uses the GPU resources that are currently notused (have been waited upon)
        //let view = Space::view(
        //    *state.back.camera_position.borrow(),
        //    *state.back.camera_rotation.borrow(),
        //);

        //let fovy = ris_math::radians(60.);
        //let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
        //let aspect_ratio = w / h;
        //let near = 0.01;
        //let far = 0.1;
        //let proj = Space::proj(fovy, aspect_ratio, near, far);

        //let proj_view = proj * view;

        //let ubo = UniformBufferObject {
        //    view,
        //    proj,
        //    proj_view,
        //};
        //self.renderer.update_uniform(image as usize, &ubo)?;

        //todo!("wanting to render");
        //let swapchain_image = &self.renderer.images[image as usize];
        //let imgui_target = ImageView::new_default(swapchain_image.clone())?;
        //let draw_data = self.imgui.backend.context().render();
        //let mut imgui_command_buffer_builder = AutoCommandBufferBuilder::primary(
        //    &self.renderer.allocators.command_buffer,
        //    self.renderer.queue.queue_family_index(),
        //    vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        //)?;
        //self.imgui.renderer.draw(
        //    imgui_target,
        //    &mut imgui_command_buffer_builder,
        //    &self.renderer.allocators,
        //    draw_data,
        //)?;
        //let imgui_command_buffer = imgui_command_buffer_builder.build()?;

        //let use_gpu_resources = false;
        //let previous_future = match self.fences[self.previous_fence].clone() {
        //    None => self.renderer.synchronize().boxed(),
        //    Some(fence) => {
        //        if use_gpu_resources {
        //            fence.wait(None)?;
        //        }

        //        fence.boxed()
        //    }
        //};

        //if use_gpu_resources {
        //    // logic that can use every GPU resource (the GPU is sleeping)
        //}

        //let fence = previous_future
        //    .join(acquire_future)
        //    .then_execute(
        //        self.renderer.queue.clone(),
        //        self.renderer.command_buffers[image as usize].clone(),
        //    )
        //    .map_err(|e| ris_error::new!("failed to execute command buffer: {}", e))?
        //    .then_execute(self.renderer.queue.clone(), imgui_command_buffer)
        //    .map_err(|e| ris_error::new!("failed to execute command buffer: {}", e))?
        //    .then_swapchain_present(
        //        self.renderer.queue.clone(),
        //        SwapchainPresentInfo::swapchain_image_index(self.renderer.swapchain.clone(), image),
        //    )
        //    .then_signal_fence_and_flush();

        //self.fences[image as usize] = match fence {
        //    Ok(fence) => {
        //        #[allow(clippy::arc_with_non_send_sync)]
        //        // false positive, since `FenceSignalFuture` indeed implements `Send`:
        //        // doc/vulkano/sync/future/struct.FenceSignalFuture.html#impl-Send-for-FenceSignalFuture%3CF%3E
        //        Some(Arc::new(fence))
        //    }
        //    Err(FlushError::OutOfDate) => {
        //        self.recreate_swapchain = true;
        //        None
        //    }
        //    Err(e) => {
        //        ris_log::warning!("failed to flush future: {}", e);
        //        None
        //    }
        //};

        //self.previous_fence = image as usize;

        Ok(())
    }
}
