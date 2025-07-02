use ash::vk::{self, Handle};

use ris_error::prelude::*;
use ris_video_data::core::VulkanCore;
use ris_video_data::swapchain::SwapchainEntry;

use crate::RendererId;

pub struct FramebufferAllocator {
    entries: Vec<Option<Entry>>,
}

#[derive(Clone)]
struct Entry {
    attachments: Vec<Vec<vk::ImageView>>,
    framebuffer: vk::Framebuffer,
}

impl FramebufferAllocator {
    pub fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            let Some(entry) = entry.take() else {
                continue;
            };

            unsafe {device.destroy_framebuffer(entry.framebuffer, None)};
        }

        self.entries.clear();
    }

    pub fn alloc() -> Self {
        Self{entries: vec![None; 4]}
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

        let entry = self.entries.get_mut(id.to_usize()).into_ris_error()?;

        let new_attachments = std::slice::from_raw_parts(
            framebuffer_create_info.p_attachments,
            framebuffer_create_info.attachment_count as usize,
        );

        match entry.as_mut() {
            Some(current_entry) => {
                let current_attachments = &current_entry.attachments.get(swapchain_index).into_ris_error()?;

                ris_error::debug_assert!(current_attachments.len() == new_attachments.len())?;

                for i in 0..current_attachments.len() {
                    let left = current_attachments[i].as_raw();
                    let right = new_attachments[i].as_raw();

                    if left == right {
                        continue;
                    }

                    // attachments changed! this usually happens when the swapchain was
                    // recreated. build a new framebuffer...

                    let attachments = new_attachments.to_vec();
                    let framebuffer = device.create_framebuffer(&framebuffer_create_info, None)?;

                    *entry = Some(Entry{
                        attachments,
                        framebuffer,
                    });

                    ris_log::trace!("recreated framebuffer");

                    return Ok(framebuffer);
                }

                Ok(current_entry.framebuffer)
            },
            None => {
                let attachments = new_attachments.to_vec();
                let framebuffer = device.create_framebuffer(&framebuffer_create_info, None)?;

                *entry = Some(Entry{
                    attachments,
                    framebuffer,
                });

                Ok(framebuffer)
            },
        }
    }
}

