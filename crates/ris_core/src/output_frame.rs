use std::ptr;

use ash::vk;
use sdl2::video::Window;
use sdl2_sys::SDL_WindowFlags;

use ris_asset::RisGodAsset;
use ris_data::gameloop::frame::Frame;
use ris_data::god_state::GodState;
use ris_data::god_state::WindowEvent;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::matrix::Mat4;
use ris_math::vector::Vec3;
use ris_video::gizmo::gizmo_renderer::GizmoRenderer;
use ris_video::imgui::RisImgui;
use ris_video::vulkan::base::VulkanBase;
use ris_video::vulkan::frame_in_flight::FrameInFlight;
use ris_video::vulkan::swapchain::BaseSwapchain;
use ris_video::vulkan::swapchain::Swapchain;
use ris_video::vulkan::swapchain::SwapchainEntry;
use ris_video::vulkan::transient_command::TransientCommandSync;

use crate::ui_helper::UiHelper;
use crate::ui_helper::UiHelperDrawData;

pub struct OutputFrame {
    current_frame: usize,
    imgui: RisImgui,
    gizmo_renderer: GizmoRenderer,
    ui_helper: UiHelper,

    // mut be dropped last
    base: VulkanBase,
    window: Window,
}

impl Drop for OutputFrame {
    fn drop(&mut self) {
        unsafe {
            ris_error::unwrap!(self.base.device.device_wait_idle(), "",);
        }

        self.imgui.renderer.free(&self.base.device);
        self.gizmo_renderer.free(&self.base.device);
        // renderer is dropped here implicitly
    }
}

impl OutputFrame {
    pub fn new(
        window: Window,
        base: VulkanBase,
        imgui: RisImgui,
        gizmo_renderer: GizmoRenderer,
        ui_helper: UiHelper,
    ) -> RisResult<Self> {
        Ok(Self {
            //recreate_swapchain: false,
            current_frame: 0,
            imgui,
            gizmo_renderer,
            ui_helper,
            base,
            window,
        })
    }

    pub fn run(
        &mut self,
        frame: Frame,
        state: &mut GodState,
        god_asset: &RisGodAsset,
    ) -> RisResult<()> {
        let window_flags = self.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;
        if is_minimized {
            return Ok(());
        }

        let mut r = ris_debug::new_record!("run output frame");

        let VulkanBase {
            device,
            graphics_queue,
            present_queue,
            swapchain:
                Swapchain {
                    base:
                        BaseSwapchain {
                            loader: swapchain_loader,
                            swapchain,
                            ..
                        },
                    entries: swapchain_entries,
                    frames_in_flight,
                    ..
                },
            ..
        } = &self.base;

        let frames_in_flight = frames_in_flight.as_ref().unroll()?;

        let FrameInFlight {
            image_available,
            render_finished,
            in_flight,
        } = &frames_in_flight[self.current_frame];
        let next_frame = (self.current_frame + 1) % frames_in_flight.len();

        let image_available_sem = [*image_available];
        let render_finished_sem = [*render_finished];

        // wait for the previous frame to finish
        ris_debug::add_record!(r, "wait for previous frame to finish")?;

        unsafe { device.wait_for_fences(&[*in_flight], true, u64::MAX) }?;
        unsafe { device.reset_fences(&[*in_flight]) }?;

        // acquire an image from the swap chain
        ris_debug::add_record!(r, "acquire an image from the swapchain")?;

        let acquire_image_result = unsafe {
            swapchain_loader.acquire_next_image(
                *swapchain,
                u64::MAX,
                *image_available,
                vk::Fence::null(),
            )
        };

        let image_index = match acquire_image_result {
            Ok((image_index, _is_sub_optimal)) => image_index,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    return self
                        .base
                        .recreate_swapchain(self.window.vulkan_drawable_size(), god_asset)
                }
                vk_result => {
                    return ris_error::new_result!("failed to acquire chain image: {}", vk_result)
                }
            },
        };

        // prepare command buffer
        let swapchain_entry = &swapchain_entries[image_index as usize];
        let SwapchainEntry {
            image_view,
            command_buffer,
            ..
        } = swapchain_entry;

        unsafe { device.reset_command_buffer(*command_buffer, vk::CommandBufferResetFlags::empty()) }?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: ptr::null(),
        };
        unsafe { device.begin_command_buffer(*command_buffer, &command_buffer_begin_info) }?;

        // update uniform buffer
        //ris_debug::add_record!(r, "update uniform buffer")?;

        //let window_drawable_size = self.window.vulkan_drawable_size();
        //let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
        //state.camera.aspect_ratio = w / h;
        //let view = state.camera.view_matrix();
        //let proj = state.camera.projection_matrix();

        //let uniform_buffer_object = UniformBufferObject {
        //    model: Mat4::init(1.0),
        //    view,
        //    proj,
        //};

        //let ubo = [uniform_buffer_object];
        //unsafe { uniform_buffer_mapped.copy_from_nonoverlapping(ubo.as_ptr(), ubo.len()) };

        // submit command buffer
        //ris_debug::add_record!(r, "submit command buffer")?;

        //let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        //let command_buffers = [*command_buffer];

        //let submit_infos = [vk::SubmitInfo {
        //    s_type: vk::StructureType::SUBMIT_INFO,
        //    p_next: ptr::null(),
        //    wait_semaphore_count: image_available_sem.len() as u32,
        //    p_wait_semaphores: image_available_sem.as_ptr(),
        //    p_wait_dst_stage_mask: wait_stages.as_ptr(),
        //    command_buffer_count: command_buffers.len() as u32,
        //    p_command_buffers: command_buffers.as_ptr(),
        //    signal_semaphore_count: main_render_sem.len() as u32,
        //    p_signal_semaphores: main_render_sem.as_ptr(),
        //}];

        //unsafe { device.queue_submit(*graphics_queue, &submit_infos, vk::Fence::null()) }?;

        // gizmos
        ris_debug::add_record!(r, "gizmos")?;

        ris_debug::add_record!(r, "draw shapes")?;
        let gizmo_shape_vertices = ris_debug::gizmo::draw_shapes(&state.camera)?;

        ris_debug::add_record!(r, "render shapes")?;
        let window_drawable_size = self.window.vulkan_drawable_size();
        self.gizmo_renderer.draw_shapes(
            &self.base,
            swapchain_entry,
            &gizmo_shape_vertices,
            window_drawable_size,
            &state.camera,
        )?;

        //ris_debug::add_record!(r, "draw text")?;
        //let (gizmo_text_vertices, gizmo_text_texture) = ris_debug::gizmo::draw_text()?;

        //ris_debug::add_record!(r, "render text")?;
        //self.gizmo_renderer.draw_text(
        //    &self.base,
        //    *image_view,
        //    &gizmo_text_vertices,
        //    &gizmo_text_texture,
        //    window_drawable_size,
        //    &state.camera,
        //    TransientCommandSync::default(),
        //)?;

        ris_debug::add_record!(r, "gizmo new frame")?;
        ris_debug::gizmo::new_frame()?;

        // ui helper
        ris_debug::add_record!(r, "prepare ui helper")?;

        let window_size = self.window.size();
        let window_drawable_size = self.window.vulkan_drawable_size();
        let imgui_ui = self.imgui.backend.prepare_frame(
            frame,
            state,
            (window_size.0 as f32, window_size.1 as f32),
            (window_drawable_size.0 as f32, window_drawable_size.1 as f32),
        );

        ris_debug::add_record!(r, "draw ui helper")?;

        self.ui_helper.draw(UiHelperDrawData {
            ui: imgui_ui,
            frame,
            state,
        })?;

        ris_debug::add_record!(r, "imgui backend")?;
        let draw_data = self.imgui.backend.context().render();
        ris_debug::add_record!(r, "imgui frontend")?;
        self.imgui.renderer.draw(
            &self.base,
            swapchain_entry,
            draw_data,
        )?;

        // end command buffer and submit
        ris_debug::add_record!(r, "submit command buffer")?;
        unsafe { device.end_command_buffer(*command_buffer) }?;
        let command_buffers = [*command_buffer];
        let wait_dst_stage_mask = [vk::PipelineStageFlags::TOP_OF_PIPE];
        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: image_available_sem.len() as u32,
            p_wait_semaphores: image_available_sem.as_ptr(),
            p_wait_dst_stage_mask: wait_dst_stage_mask.as_ptr(),
            command_buffer_count: command_buffers.len() as u32,
            p_command_buffers: command_buffers.as_ptr(),
            signal_semaphore_count: render_finished_sem.len() as u32,
            p_signal_semaphores: render_finished_sem.as_ptr(),
        }];

        unsafe { device.queue_submit(*graphics_queue, &submit_infos, *in_flight) }?;
 
        // present swap chain image
        ris_debug::add_record!(r, "present the swap chain image")?;
        let swapchains = [*swapchain];

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: render_finished_sem.len() as u32,
            p_wait_semaphores: render_finished_sem.as_ptr(),
            swapchain_count: swapchains.len() as u32,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
        };

        let queue_present_result =
            unsafe { swapchain_loader.queue_present(*present_queue, &present_info) };

        // recreate swapchain
        let window_event = match queue_present_result {
            Ok(_) => match state.window_event {
                WindowEvent::SizeChanged(..) => state.window_event,
                WindowEvent::None => {
                    if state.reload_shaders {
                        let (width, height) = self.window.vulkan_drawable_size();
                        WindowEvent::SizeChanged(width, height)
                    } else {
                        WindowEvent::None
                    }
                }
            },
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => {
                    let (width, height) = self.window.vulkan_drawable_size();
                    WindowEvent::SizeChanged(width, height)
                }
                vk_result => {
                    return ris_error::new_result!("failed to present queue: {}", vk_result)
                }
            },
        };

        if let WindowEvent::SizeChanged(width, height) = window_event {
            self.base.recreate_swapchain((width, height), god_asset)?;
        }

        self.current_frame = next_frame;

        ris_debug::end_record!(r)?;

        Ok(())
    }
}
