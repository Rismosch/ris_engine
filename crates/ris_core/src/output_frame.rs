use std::ptr;

use ash::vk;
use sdl2::video::Window;
use sdl2_sys::SDL_WindowFlags;

use ris_asset::RisGodAsset;
use ris_data::gameloop::frame::Frame;
use ris_data::god_state::GodState;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_video::gizmo::gizmo_segment_renderer::GizmoSegmentRenderer;
use ris_video::gizmo::gizmo_text_renderer::GizmoTextRenderer;
use ris_video::imgui::imgui_backend::ImguiBackend;
use ris_video::imgui::imgui_renderer::ImguiRenderer;
use ris_video::scene::scene_renderer::SceneRenderer;
use ris_video::vulkan::core::VulkanCore;
use ris_video::vulkan::frame_in_flight::FrameInFlight;
use ris_video::vulkan::swapchain::SwapchainEntry;

use crate::ui_helper::UiHelper;
use crate::ui_helper::UiHelperDrawData;

pub struct Renderer{
    pub scene: SceneRenderer,
    pub gizmo_segment: GizmoSegmentRenderer,
    pub gizmo_text: GizmoTextRenderer,
    pub imgui: ImguiRenderer,
}

pub struct OutputFrame {
    current_frame: usize,
    renderer: Renderer,
    imgui_backend: ImguiBackend,
    ui_helper: UiHelper,

    // mut be dropped last
    core: VulkanCore,
    window: Window,
}

impl Drop for OutputFrame {
    fn drop(&mut self) {
        unsafe {
        if let Err(e) = self.core.device.device_wait_idle() {
            ris_log::fatal!("cannot clean up output frame. device_wait_idle failed: {}", e);

            return;
        }

            let device = &self.core.device;

            self.renderer.scene.free(device);
            self.renderer.gizmo_segment.free(device);
            self.renderer.gizmo_text.free(device);
            self.renderer.imgui.free(device);

            self.core.free();
        }
    }
}

impl OutputFrame {
    pub fn new(
        window: Window,
        core: VulkanCore,
        renderer: Renderer,
        imgui_backend: ImguiBackend,
        ui_helper: UiHelper,
    ) -> RisResult<Self> {
        Ok(Self {
            current_frame: 0,
            renderer,
            imgui_backend,
            ui_helper,
            core,
            window,
        })
    }

    pub fn run(&mut self, frame: Frame, state: &mut GodState, god_asset: &RisGodAsset) -> RisResult<()> {
        let window_flags = self.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;
        if is_minimized {
            return Ok(());
        }

        let mut r = ris_debug::new_record!("run output frame");

        let VulkanCore {
            device,
            graphics_queue,
            present_queue,
            swapchain,
            ..
        } = &self.core;

        let frames_in_flight = swapchain.frames_in_flight.as_ref().unroll()?;
        let FrameInFlight {
            image_available,
            render_finished,
            in_flight,
        } = &frames_in_flight[self.current_frame];
        let next_frame = (self.current_frame + 1) % frames_in_flight.len();
        self.current_frame = next_frame;

        let image_available_sem = [*image_available];
        let render_finished_sem = [*render_finished];

        // wait for the previous frame to finish
        ris_debug::add_record!(r, "wait for previous frame to finish")?;

        unsafe { device.wait_for_fences(&[*in_flight], true, u64::MAX) }?;
        unsafe { device.reset_fences(&[*in_flight]) }?;

        // rebuild renderers
        ris_debug::add_record!(r, "rebuild renderers")?;
        if state.event_rebuild_renderers {
            unsafe {
                ris_log::trace!("rebuilding renderers...");
                self.core.device.device_wait_idle()?;

                self.renderer.scene.free(device);
                self.renderer.gizmo_segment.free(device);
                self.renderer.gizmo_text.free(device);
                self.renderer.imgui.free(device);

                self.renderer.scene = SceneRenderer::alloc(&self.core, god_asset)?;
                self.renderer.gizmo_segment = GizmoSegmentRenderer::alloc(&self.core, god_asset)?;
                self.renderer.gizmo_text = GizmoTextRenderer::alloc(&self.core, god_asset)?;
                self.renderer.imgui = ImguiRenderer::alloc(&self.core, god_asset, self.imgui_backend.context())?;

                ris_log::debug!("rebuilt renderers!");
            }
        }

        // acquire an image from the swap chain
        ris_debug::add_record!(r, "acquire an image from the swapchain")?;

        let acquire_image_result = unsafe {
            swapchain.loader.acquire_next_image(
                swapchain.swapchain,
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
                        .core
                        .recreate_swapchain(self.window.vulkan_drawable_size())
                }
                vk_result => {
                    return ris_error::new_result!("failed to acquire chain image: {}", vk_result)
                }
            },
        };

        // prepare command buffer
        ris_debug::add_record!(r, "prepare command buffer")?;
        let swapchain_entry = &swapchain.entries[image_index as usize];
        let SwapchainEntry { command_buffer, .. } = swapchain_entry;

        unsafe {
            device.reset_command_buffer(*command_buffer, vk::CommandBufferResetFlags::empty())
        }?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: ptr::null(),
        };
        unsafe { device.begin_command_buffer(*command_buffer, &command_buffer_begin_info) }?;

        // prepare camera
        ris_debug::add_record!(r, "prepare camera")?;
        let window_drawable_size = self.window.vulkan_drawable_size();
        let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
        state.camera.aspect_ratio = w / h;

        // scene
        ris_debug::add_record!(r, "scene")?;
        let vertices = ris_video::scene::scene_mesh::VERTICES;
        let indices = ris_video::scene::scene_mesh::INDICES;
        self.renderer.scene.draw(
            &self.core,
            swapchain_entry,
            &vertices,
            &indices,
            window_drawable_size,
            &state.camera,
        )?;

        // gizmos
        ris_debug::add_record!(r, "gizmos")?;

        ris_debug::add_record!(r, "get segment vertices")?;
        let gizmo_segment_vertices = ris_debug::gizmo::draw_segments(&state.camera)?;

        ris_debug::add_record!(r, "draw segments")?;
        self.renderer.gizmo_segment.draw(
            &self.core,
            swapchain_entry,
            &gizmo_segment_vertices,
            window_drawable_size,
            &state.camera,
        )?;

        ris_debug::add_record!(r, "get text vertices")?;
        let (gizmo_text_vertices, gizmo_text_texture) = ris_debug::gizmo::draw_text()?;

        ris_debug::add_record!(r, "draw text")?;
        self.renderer.gizmo_text.draw(
            &self.core,
            swapchain_entry,
            &gizmo_text_vertices,
            &gizmo_text_texture,
            window_drawable_size,
            &state.camera,
        )?;

        ris_debug::add_record!(r, "gizmo new frame")?;
        ris_debug::gizmo::new_frame()?;

        // ui helper
        ris_debug::add_record!(r, "prepare ui helper")?;

        let window_size = self.window.size();
        let window_drawable_size = self.window.vulkan_drawable_size();
        let imgui_ui = self.imgui_backend.prepare_frame(
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
        let draw_data = self.imgui_backend.context().render();
        ris_debug::add_record!(r, "imgui frontend")?;
        self.renderer
            .imgui
            .draw(&self.core, swapchain_entry, draw_data)?;

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
        let swapchains = [swapchain.swapchain];

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

        let queue_present_result = unsafe {
            swapchain
                .loader
                .queue_present(*present_queue, &present_info)
        };

        // recreate swapchain
        let event_window_resized = match queue_present_result {
            Ok(_) => state.event_window_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => {
                    Some(self.window.vulkan_drawable_size())
                }
                vk_result => {
                    return ris_error::new_result!("failed to present queue: {}", vk_result)
                }
            },
        };

        if let Some((width, height)) = event_window_resized {
            self.core.recreate_swapchain((width, height))?;
        }

        ris_debug::end_record!(r)?;
        Ok(())
    }
}
