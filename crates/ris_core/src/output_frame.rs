use core::option::Option::None;
use std::ptr;

use ash::vk;
use ris_log::debug;
use sdl2::video::Window;
use sdl2_sys::SDL_WindowFlags;

use ris_asset::RisGodAsset;
use ris_asset::lookup::ris_mesh_lookup::MeshLookup;
use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::GodState;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_video_data::core::VulkanCore;
use ris_video_data::frames_in_flight::FrameInFlight;
use ris_video_data::frames_in_flight::FrameInFlightCreateInfo;
use ris_video_data::frames_in_flight::FramesInFlight;
use ris_video_data::frames_in_flight::RendererId;
use ris_video_data::frames_in_flight::RendererRegisterer;
use ris_video_data::swapchain::SwapchainEntry;
use ris_video_renderers::GizmoSegmentRenderer;
use ris_video_renderers::GizmoTextRenderer;
use ris_video_renderers::SceneRenderer;
use ris_video_renderers::SceneRendererArgs;
#[cfg(feature = "ui_helper_enabled")]
use ris_video_renderers::{ImguiBackend, ImguiRenderer, ImguiRendererArgs};

#[cfg(feature = "ui_helper_enabled")]
use crate::ui_helper::{UiHelper, UiHelperDrawData};

pub struct Renderer {
    pub scene: SceneRenderer,
    pub gizmo_segment: GizmoSegmentRenderer,
    pub gizmo_text: GizmoTextRenderer,
    #[cfg(feature = "ui_helper_enabled")]
    pub imgui: ImguiRenderer,
    pub frames_in_flight: Option<FramesInFlight>,
}

pub struct RendererIds {
    pub scene: RendererId,
    pub gizmo_segment: RendererId,
    pub gizmo_text: RendererId,
    #[cfg(feature = "ui_helper_enabled")]
    pub imgui: RendererId,
}

impl Renderer {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(
        &mut self,
        device: &ash::Device,
        free_frames_in_flight: bool,
    ) {
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
        #[cfg(feature = "ui_helper_enabled")]
        imgui_context: &mut imgui::Context,
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
        #[cfg(feature = "ui_helper_enabled")]
        imgui_context: &mut imgui::Context,
        frames_in_flight: Option<FramesInFlight>,
    ) -> RisResult<Self> {
        let frame_in_flight_create_info = FrameInFlightCreateInfo {
            suitable_device: &core.suitable_device,
            device: &core.device,
            renderer_count: 0,
            command_buffer_count: 0,
        };

        let mut renderer_registerer = RendererRegisterer {
            info: frame_in_flight_create_info,
            existing_id: None,
        };

        renderer_registerer.existing_id = renderer_ids.map(|x| x.scene.clone());
        let scene = SceneRenderer::alloc(
            core,
            god_asset,
            mesh_lookup,
            &mut renderer_registerer,
        )?;
        let gizmo_segment = GizmoSegmentRenderer::alloc(
            core,
            god_asset,
            &mut renderer_registerer,
        )?;
        let gizmo_text = GizmoTextRenderer::alloc(
            core,
            god_asset,
            &mut renderer_registerer,
        )?;
        #[cfg(feature = "ui_helper_enabled")]
        let imgui = ImguiRenderer::alloc(
            core,
            god_asset,
            imgui_context,
            &mut renderer_registerer,
        )?;

        let frames_in_flight = if let Some(frames_in_flight) = frames_in_flight {
            Some(frames_in_flight)
        } else {
            Some(FramesInFlight::alloc(renderer_registerer.info)?)
        };

        Ok(Self{
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
        #[cfg(feature = "ui_helper_enabled")]
        imgui_context: &mut imgui::Context,
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
        mesh_lookup.reimport_everything(
            device,
            physical_device_memory_properties,
        );

        let renderer_ids = RendererIds {
            scene: self.scene.renderer_id.clone(),
            gizmo_segment: self.gizmo_segment.renderer_id.clone(),
            gizmo_text: self.gizmo_text.renderer_id.clone(),
            #[cfg(feature = "ui_helper_enabled")]
            imgui: self.imgui.renderer_id.clone(),
        };

        let frames_in_flight = self.frames_in_flight.take();

        unsafe {self.free(device, false)};
        *self = Self::alloc_internal(
            core,
            god_asset,
            Some(mesh_lookup),
            Some(renderer_ids),
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

        let VulkanCore {
            instance,
            suitable_device,
            device,
            graphics_queue,
            present_queue,
            swapchain,
            ..
        } = &self.core;

        //let frames_in_flight = swapchain.frames_in_flight.as_ref().into_ris_error()?;
        //let FrameInFlight {
        //    image_available,
        //    render_finished,
        //    in_flight,
        //} = &frames_in_flight[self.current_frame];
        //let next_frame = (self.current_frame + 1) % frames_in_flight.len();
        //self.current_frame = next_frame;

        //let image_available_sem = [*image_available];
        //let render_finished_sem = [*render_finished];

        //// wait for the previous frame to finish
        //ris_debug::add_record!(r, "wait for previous frame to finish")?;

        //unsafe { device.wait_for_fences(&[*in_flight], true, u64::MAX) }?;
        //unsafe { device.reset_fences(&[*in_flight]) }?;


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
                    &god_asset,
                    #[cfg(feature = "ui_helper_enabled")]
                    self.imgui_backend.context()
                )?;
                ris_log::debug!("rebuilt renderers!");
            }
        }

        // dvance frame in flight
        let FrameInFlight { 
            index,
            command_pool,
            primary_command_buffer,
            secondary_command_buffers,
            image_available,
            renderer_finished,
            command_buffer_finished,
            done,
        } = &mut self.renderer
            .frames_in_flight
            .as_mut()
            .into_ris_error()?
            .acquire_next_frame(device)?;

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
                    self.core
                        .recreate_swapchain(self.window.vulkan_drawable_size())?;
                    return Ok(ui_helper_state);
                }
                vk_result => {
                    return ris_error::new_result!("failed to acquire chain image: {}", vk_result)
                }
            },
        };

        // prepare command buffer
        ris_debug::add_record!(r, "prepare command buffer")?;
        let swapchain_entry = &swapchain.entries[image_index as usize];

        unsafe {device.reset_command_pool(
            *command_pool,
            vk::CommandPoolResetFlags::empty(),
        )};

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: ptr::null(),
        };
        unsafe { device.begin_command_buffer(*primary_command_buffer, &command_buffer_begin_info) }?;

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
        };

        self.renderer.scene.draw(args)?;

        // gizmos
        ris_debug::add_record!(r, "gizmos")?;
        let gizmo_segment_vertices = ris_debug::gizmo::draw_segments(&camera)?;

        let (gizmo_text_vertices, gizmo_text_texture) = ris_debug::gizmo::draw_text()?;

        self.renderer.gizmo_text.draw(
            &self.core,
            swapchain_entry,
            &gizmo_text_vertices,
            &gizmo_text_texture,
            window_drawable_size,
            &camera,
        )?;

        self.renderer.gizmo_segment.draw(
            &self.core,
            swapchain_entry,
            &gizmo_segment_vertices,
            window_drawable_size,
            &camera,
        )?;

        ris_debug::gizmo::new_frame()?;

        // imgui
        #[cfg(feature = "ui_helper_enabled")]
        {
            ris_debug::add_record!(r, "imgui backend")?;
            let draw_data = self.imgui_backend.context().render();

            ris_debug::add_record!(r, "imgui frontend")?;
            let args = ImguiRendererArgs {
                core: &self.core,
                swapchain_entry,
                draw_data,
            };

            self.renderer.imgui.draw(args)?;
        }

        // end command buffer and submit
        ris_debug::add_record!(r, "submit command buffer")?;
        unsafe { device.end_command_buffer(*primary_command_buffer) }?;
        let command_buffers = [*primary_command_buffer];
        let wait_dst_stage_mask = [vk::PipelineStageFlags::TOP_OF_PIPE];
        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: image_available,
            p_wait_dst_stage_mask: wait_dst_stage_mask.as_ptr(),
            command_buffer_count: command_buffers.len() as u32,
            p_command_buffers: command_buffers.as_ptr(),
            signal_semaphore_count: 1,
            p_signal_semaphores: command_buffer_finished,
        }];

        unsafe { device.queue_submit(*graphics_queue, &submit_infos, *done) }?;

        // present swap chain image
        ris_debug::add_record!(r, "present the swap chain image")?;
        let swapchains = [swapchain.swapchain];

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: command_buffer_finished,
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
        Ok(ui_helper_state)
    }
}
