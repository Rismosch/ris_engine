use ash::vk::{self, Handle};

use ris_error::prelude::*;
use ris_video_data::core::VulkanCore;
use ris_video_data::swapchain::SwapchainEntry;

use crate::RendererId;

pub struct FramebufferAllocator {
    // one framebuffer for each renderer and each swapchain entry
    entries: Vec<Vec<Option<FrameBufferAllocatorEntry>>>,
}

struct FrameBufferAllocatorEntry {
    attachments: Vec<vk::ImageView>,
    framebuffer: vk::Framebuffer,
}

impl FramebufferAllocator {
    pub fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            for entry in entry.iter_mut(){
                let Some(entry) = entry.take() else {
                    continue;
                };

                unsafe {device.destroy_framebuffer(entry.framebuffer, None)};
            }
        }

        self.entries.clear();
    }

    pub fn alloc(swapchain_entry_count: usize) -> Self {
        let renderers = 4;
        let mut renderer_entries = Vec::with_capacity(renderers);

        for _ in 0..renderers {
            let mut swapchain_entries = Vec::with_capacity(swapchain_entry_count);
            for _ in 0..swapchain_entry_count {
                swapchain_entries.push(None);
            }

            renderer_entries.push(swapchain_entries);
        }

        Self{
            entries: renderer_entries,
        }
    }

    /// # Safety
    ///
    /// Caller must guarantee that the vk::FramebufferCreateInfo is properly constructed. Otherwise
    /// usual care when dealing with Vulkan objects.
    pub unsafe fn get<'a>(
        &mut self,
        id: RendererId,
        device: &ash::Device,
        framebuffer_create_info: vk::FramebufferCreateInfo,
        swapchain_index: usize,
    ) -> RisResult<vk::Framebuffer> {

        let entry = self.entries
            .get_mut(id.to_usize()).into_ris_error()?
            .get_mut(swapchain_index).into_ris_error()?;

        let new_attachments = std::slice::from_raw_parts(
            framebuffer_create_info.p_attachments,
            framebuffer_create_info.attachment_count as usize,
        );

        //if let Some(entry) = entry.take() {
        //    device.destroy_framebuffer(entry.framebuffer, None);
        //}

        //let attachments = new_attachments.to_vec();
        //let framebuffer = device.create_framebuffer(&framebuffer_create_info, None)?;

        //*entry = Some(FrameBufferAllocatorEntry{
        //    attachments,
        //    framebuffer,
        //});

        //return Ok(framebuffer);

        if let Some(current_entry) = entry.as_mut() {
            let current_attachments = &current_entry.attachments;

            ris_error::debug_assert!(current_attachments.len() == new_attachments.len())?;

            let mut build_new_framebuffer = false;
            for i in 0..current_attachments.len() {
                let left = current_attachments[i].as_raw();
                let right = new_attachments[i].as_raw();

                if left != right {
                    // attachments changed! this usually happens when the swapchain was
                    // recreated. build a new framebuffer...
                    device.destroy_framebuffer(current_entry.framebuffer, None);

                    build_new_framebuffer = true;
                    break;
                }
            }

            if !build_new_framebuffer {
                return Ok(current_entry.framebuffer);
            }
        };

        // build new frame buffer...
        let attachments = new_attachments.to_vec();
        let framebuffer = device.create_framebuffer(&framebuffer_create_info, None)?;

        *entry = Some(FrameBufferAllocatorEntry{
            attachments,
            framebuffer,
        });

        ris_log::trace!("recreated framebuffer");

        return Ok(framebuffer);
    }
}

