use core::option::Option::None;
use std::ptr;

use ash::vk;
use sdl2::video::Window;
use sdl2_sys::SDL_WindowFlags;

use ris_asset::lookup::ris_mesh_lookup::MeshLookup;
use ris_asset::RisGodAsset;
use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::GodState;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_video_data::core::VulkanCore;
use ris_video_data::frames_in_flight::FrameInFlightCreateInfo;
use ris_video_data::frames_in_flight::FramesInFlight;
use ris_video_data::frames_in_flight::RendererId;
use ris_video_data::frames_in_flight::RendererRegisterer;
use ris_video_renderers::GizmoSegmentRenderer;
use ris_video_renderers::GizmoSegmentRendererArgs;
use ris_video_renderers::GizmoTextRenderer;
use ris_video_renderers::GizmoTextRendererArgs;
use ris_video_renderers::SceneRenderer;
use ris_video_renderers::SceneRendererArgs;
#[cfg(feature = "ui_helper_enabled")]
use ris_video_renderers::{ImguiBackend, ImguiRenderer, ImguiRendererArgs};

#[cfg(feature = "ui_helper_enabled")]
use crate::ui_helper::{UiHelper, UiHelperDrawData};

pub struct Renderer {
    count: usize,
    scene: SceneRenderer,
    gizmo_segment: GizmoSegmentRenderer,
    gizmo_text: GizmoTextRenderer,
    #[cfg(feature = "ui_helper_enabled")]
    imgui: ImguiRenderer,
    frames_in_flight: Option<FramesInFlight>,
}

pub struct RendererIds {
    scene: RendererId,
    gizmo_segment: RendererId,
    gizmo_text: RendererId,
    #[cfg(feature = "ui_helper_enabled")]
    imgui: RendererId,
}

impl Renderer {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&mut self, device: &ash::Device, free_frames_in_flight: bool) {
        if free_frames_in_flight {
            if let Some(mut frames_in_flight) = self.frames_in_flight.take() {
                frames_in_flight.free(device);
            }
        }

        self.scene.free(device);
        self.gizmo_segment.free(device);
        self.gizmo_text.free(device);
        #[cfg(feature = "ui_helper_enabled")]
        self.imgui.free(device);
    }

    pub fn alloc(
        core: &VulkanCore,
        god_asset: &RisGodAsset,
        #[cfg(feature = "ui_helper_enabled")] imgui_context: &mut imgui::Context,
    ) -> RisResult<Self> {
        Self::alloc_internal(
            core,
            god_asset,
            None,
            None,
            #[cfg(feature = "ui_helper_enabled")]
            imgui_context,
            None,
        )
    }

    fn alloc_internal(
        core: &VulkanCore,
        god_asset: &RisGodAsset,
        mesh_lookup: Option<MeshLookup>,
        renderer_ids: Option<RendererIds>,
        #[cfg(feature = "ui_helper_enabled")] imgui_context: &mut imgui::Context,
        frames_in_flight: Option<FramesInFlight>,
    ) -> RisResult<Self> {
        let frame_in_flight_create_info = FrameInFlightCreateInfo {
            suitable_device: &core.suitable_device,
            device: &core.device,
            renderer_count: 0,
            secondary_command_buffer_count: 0,
        };

        let mut renderer_registerer = RendererRegisterer {
            info: frame_in_flight_create_info,
            existing_id: None,
        };

        renderer_registerer.existing_id = renderer_ids.as_ref().map(|x| x.scene);
        let scene = SceneRenderer::alloc(core, god_asset, mesh_lookup, &mut renderer_registerer)?;
        renderer_registerer.existing_id = renderer_ids.as_ref().map(|x| x.gizmo_segment);
        let gizmo_segment = GizmoSegmentRenderer::alloc(core, god_asset, &mut renderer_registerer)?;
        renderer_registerer.existing_id = renderer_ids.as_ref().map(|x| x.gizmo_text);
        let gizmo_text = GizmoTextRenderer::alloc(core, god_asset, &mut renderer_registerer)?;

        #[cfg(feature = "ui_helper_enabled")]
        let imgui = {
            renderer_registerer.existing_id = renderer_ids.as_ref().map(|x| x.imgui);
            ImguiRenderer::alloc(core, god_asset, imgui_context, &mut renderer_registerer)?
        };

        let renderer_count = renderer_registerer.info.renderer_count;

        let frames_in_flight = if let Some(frames_in_flight) = frames_in_flight {
            Some(frames_in_flight)
        } else {
            Some(FramesInFlight::alloc(renderer_registerer.info)?)
        };

        Ok(Self {
            count: renderer_count,
            scene,
            gizmo_segment,
            gizmo_text,
            #[cfg(feature = "ui_helper_enabled")]
            imgui,
            frames_in_flight,
        })
    }

    fn recreate(
        &mut self,
        core: &VulkanCore,
        god_asset: &RisGodAsset,
        #[cfg(feature = "ui_helper_enabled")] imgui_context: &mut imgui::Context,
    ) -> RisResult<()> {
        let VulkanCore {
            instance,
            suitable_device,
            device,
            ..
        } = core;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let mut mesh_lookup = self.scene.mesh_lookup.take().into_ris_error()?;
        mesh_lookup.reimport_everything(device, physical_device_memory_properties);

        let renderer_ids = RendererIds {
            scene: self.scene.renderer_id,
            gizmo_segment: self.gizmo_segment.renderer_id,
            gizmo_text: self.gizmo_text.renderer_id,
            #[cfg(feature = "ui_helper_enabled")]
            imgui: self.imgui.renderer_id,
        };

        let frames_in_flight = self.frames_in_flight.take();

        unsafe { self.free(device, false) };
        *self = Self::alloc_internal(
            core,
            god_asset,
            Some(mesh_lookup),
            Some(renderer_ids),
            #[cfg(feature = "ui_helper_enabled")]
            imgui_context,
            frames_in_flight,
        )?;

        Ok(())
    }
}

pub struct OutputFrame {
    pub renderer: Renderer,
    #[cfg(feature = "ui_helper_enabled")]
    pub imgui_backend: ImguiBackend,
    #[cfg(feature = "ui_helper_enabled")]
    pub ui_helper: UiHelper,

    // must be dropped last
    pub core: VulkanCore,
    pub window: Window,
}

impl Drop for OutputFrame {
    fn drop(&mut self) {
        unsafe {
            if let Err(e) = self.wait_idle() {
                ris_log::fatal!(
                    "cannot clean up output frame. device_wait_idle failed: {}",
                    e
                );

                return;
            }

            self.renderer.free(&self.core.device, true);
            self.core.free();
        }
    }
}

impl OutputFrame {
    pub fn wait_idle(&self) -> RisResult<()> {
        unsafe { self.core.device.device_wait_idle() }?;
        Ok(())
    }

    pub fn run(
        &mut self,
        frame: Frame,
        state: &mut GodState,
        god_asset: &RisGodAsset,
    ) -> RisResult<GameloopState> {
        let window_flags = self.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;
        if is_minimized {
            std::thread::sleep(std::time::Duration::from_millis((1000.0 / 15.0) as u64)); // 15 fps
            return Ok(GameloopState::WantsToContinue);
        }

        let mut r = ris_debug::new_record!("run output frame");

        // ui helper
        let ui_helper_state = {
            #[cfg(feature = "ui_helper_enabled")]
            {
                ris_debug::add_record!(r, "ui helper")?;

                let window_size = self.window.size();
                let window_drawable_size = self.window.vulkan_drawable_size();
                let imgui_ui = self.imgui_backend.prepare_frame(
                    frame,
                    state,
                    (window_size.0 as f32, window_size.1 as f32),
                    (window_drawable_size.0 as f32, window_drawable_size.1 as f32),
                )?;

                self.ui_helper.draw(UiHelperDrawData {
                    ui: imgui_ui,
                    frame,
                    state,
                    window_drawable_size,
                })?
            }

            #[cfg(not(feature = "ui_helper_enabled"))]
            {
                let _ = frame;
                GameloopState::WantsToContinue
            }
        };

        // rebuild renderers
        ris_debug::add_record!(r, "rebuild renderers")?;
        if state.event_rebuild_renderers {
            unsafe {
                ris_log::trace!("rebuilding renderers...");
                self.core.device.device_wait_idle()?;
                self.renderer.recreate(
                    &self.core,
                    god_asset,
                    #[cfg(feature = "ui_helper_enabled")]
                    self.imgui_backend.context(),
                )?;
                ris_log::debug!("rebuilt renderers!");
            }
        }

        let device = self.core.device.clone();
        let graphics_queue = self.core.graphics_queue;
        let present_queue = self.core.present_queue;

        // advance frame in flight
        let frame_in_flight = self
            .renderer
            .frames_in_flight
            .as_mut()
            .into_ris_error()?
            .acquire_next_frame(&device)?;

        // acquire an image from the swap chain
        ris_debug::add_record!(r, "acquire an image from the swapchain")?;
        let acquire_image_result = unsafe {
            self.core.swapchain.loader.acquire_next_image(
                self.core.swapchain.swapchain,
                u64::MAX,
                frame_in_flight.image_available,
                vk::Fence::null(),
            )
        };

        let image_index = match acquire_image_result {
            Ok((image_index, _is_sub_optimal)) => image_index,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    self.core
                        .recreate_swapchain(self.window.vulkan_drawable_size())?;
                    return Ok(ui_helper_state);
                }
                vk_result => {
                    return ris_error::new_result!("failed to acquire chain image: {}", vk_result)
                }
            },
        };

        self.core
            .swapchain
            .reserve_framebuffers(image_index as usize, self.renderer.count);
        let swapchain_entry = &self.core.swapchain.entries[image_index as usize];

        // prepare command buffers
        ris_debug::add_record!(r, "prepare command buffer")?;
        unsafe {
            device.reset_command_pool(
                frame_in_flight.command_pool,
                vk::CommandPoolResetFlags::empty(),
            )
        }?;

        // prepare camera
        ris_debug::add_record!(r, "prepare camera")?;
        let window_drawable_size = self.window.vulkan_drawable_size();
        let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
        state.camera.borrow_mut().aspect_ratio = w / h;
        let camera = state.camera.borrow().clone();

        // scene
        ris_debug::add_record!(r, "scene")?;
        let args = SceneRendererArgs {
            core: &self.core,
            swapchain_entry,
            window_drawable_size,
            camera: &camera,
            scene: &state.scene,
            frame_in_flight,
        };

        let scene_command_buffer = self.renderer.scene.draw(args)?;

        // gizmos
        ris_debug::add_record!(r, "gizmos")?;
        let gizmo_segment_vertices = ris_debug::gizmo::draw_segments(&camera)?;

        let (gizmo_text_vertices, gizmo_text_texture) = ris_debug::gizmo::draw_text()?;

        let args = GizmoTextRendererArgs {
            core: &self.core,
            swapchain_entry,
            vertices: &gizmo_text_vertices,
            text: &gizmo_text_texture,
            window_drawable_size,
            camera: &camera,
            frame_in_flight,
        };
        let gizmo_text_command_buffer = self.renderer.gizmo_text.draw(args)?;

        let args = GizmoSegmentRendererArgs {
            core: &self.core,
            swapchain_entry,
            vertices: &gizmo_segment_vertices,
            window_drawable_size,
            camera: &camera,
            frame_in_flight,
        };
        let gizmo_segment_command_buffer = self.renderer.gizmo_segment.draw(args)?;

        ris_debug::gizmo::new_frame()?;

        // imgui
        #[cfg(feature = "ui_helper_enabled")]
        let imgui_command_buffer = {
            ris_debug::add_record!(r, "imgui backend")?;
            let draw_data = self.imgui_backend.context().render();

            ris_debug::add_record!(r, "imgui frontend")?;
            let args = ImguiRendererArgs {
                core: &self.core,
                swapchain_entry,
                draw_data,
                frame_in_flight,
            };

            self.renderer.imgui.draw(args)?
        };

        // end command buffer and submit
        ris_debug::add_record!(r, "submit command buffer")?;

        let mut command_buffers = vec![scene_command_buffer];
        let mut enqueue_command_buffer = |command_buffer: Option<vk::CommandBuffer>| {
            if let Some(command_buffer) = command_buffer {
                command_buffers.push(command_buffer);
            }
        };
        enqueue_command_buffer(gizmo_text_command_buffer);
        enqueue_command_buffer(gizmo_segment_command_buffer);
        #[cfg(feature = "ui_helper_enabled")]
        enqueue_command_buffer(imgui_command_buffer);

        let wait_dst_stage_mask = [vk::PipelineStageFlags::TOP_OF_PIPE];
        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: &frame_in_flight.image_available,
            p_wait_dst_stage_mask: wait_dst_stage_mask.as_ptr(),
            command_buffer_count: command_buffers.len() as u32,
            p_command_buffers: command_buffers.as_ptr(),
            signal_semaphore_count: 1,
            p_signal_semaphores: &frame_in_flight.finished_semaphore,
        }];

        unsafe {
            device.queue_submit(
                graphics_queue,
                &submit_infos,
                frame_in_flight.finished_fence,
            )
        }?;

        // present swap chain image
        ris_debug::add_record!(r, "present the swap chain image")?;

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: &frame_in_flight.finished_semaphore,
            swapchain_count: 1,
            p_swapchains: &self.core.swapchain.swapchain,
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
        };

        let queue_present_result = unsafe {
            self.core
                .swapchain
                .loader
                .queue_present(present_queue, &present_info)
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
        Ok(ui_helper_state)
    }
}
