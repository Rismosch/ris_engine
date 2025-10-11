use std::ptr;

use ash::vk;

use ris_error::prelude::*;

use super::suitable_device::SuitableDevice;

// with no frames in flight (MAX_FRAMES_IN_FLIGHT == 1), we
// always need to wait for the command buffers to be done
// executing before recording them again. recording may be
// expensive, thus leaving the GPU to run idle. to maximize
// hardware usage, we allow recording of new command buffers,
// while previous ones are being executed. higher values for
// MAX_FRAMES_IN_FLIGHT allow more parallelism, but also more
// frames produce more latency, as the GPU may lag behind the
// CPU.
//
// no frames in flight provide the lowest latency
const MAX_FRAMES_IN_FLIGHT: usize = 2;
const _: () = {
    assert!(
        MAX_FRAMES_IN_FLIGHT > 0,
        "MAX_FRAMES_IN_FLIGHT may not be 0",
    )
};

pub struct FramesInFlight {
    pub entries: Vec<FrameInFlight>,
    pub current_frame: usize,
}

pub struct FrameInFlight {
    pub index: usize,

    // rendering related
    pub command_pool: vk::CommandPool,
    pub primary_command_buffer: vk::CommandBuffer,
    pub secondary_command_buffers: Vec<vk::CommandBuffer>, // multiple per renderer
    pub framebuffers: Vec<Option<vk::Framebuffer>>, // one per renderer

    // synchronization
    pub image_available: vk::Semaphore,
    pub renderer_finished: Vec<vk::Semaphore>,
    pub command_buffer_finished: vk::Semaphore,
    pub done: vk::Fence,
}

#[derive(Debug, Clone, PartialEq, Eq)] // 
pub struct RendererId {
    index: usize,
    command_buffers_start: usize,
    command_buffers_end: usize,
}

pub struct FrameInFlightCreateInfo<'a> {
    suitable_device: &'a SuitableDevice,
    device: &'a ash::Device,
    renderer_count: usize,
    command_buffer_count: usize,
}

impl<'a> FrameInFlightCreateInfo<'a> {
    pub fn register_renderer(&mut self, command_buffer_count: usize) -> RendererId {
        let index = self.renderer_count;
        let command_buffers_start = self.command_buffer_count;
        let command_buffers_end = command_buffers_start + command_buffer_count;

        self.renderer_count += 1;
        self.command_buffer_count += command_buffer_count;

        RendererId {
            index,
            command_buffers_start,
            command_buffers_end,
        }
    }
}

impl FramesInFlight {
    pub unsafe fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            // free synchronization
            device.destroy_fence(
                entry.done,
                None,
            );

            device.destroy_semaphore(
                entry.command_buffer_finished,
                None,
            );

            for &semaphore in entry.renderer_finished.iter() {
                device.destroy_semaphore(
                    semaphore,
                    None,
                );
            }
            device.destroy_semaphore(
                entry.image_available,
                None,
            );
            
            // free frame buffers
            for framebuffer in entry.framebuffers.iter_mut() {
                if let Some(framebuffer) = framebuffer.take() {
                    device.destroy_framebuffer(framebuffer, None);
                }
            }

            // free command buffers
            device.free_command_buffers(entry.command_pool, &entry.secondary_command_buffers);
            device.free_command_buffers(entry.command_pool, &[entry.primary_command_buffer]);
            device.destroy_command_pool(entry.command_pool, None);
        }

        self.entries.clear();
    }

    pub fn alloc(info: FrameInFlightCreateInfo) -> RisResult<Self> {
        let FrameInFlightCreateInfo {
            suitable_device,
            device,
            renderer_count,
            command_buffer_count,
        } = info;


        let mut entries = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
        for i in 0..entries.capacity() {
            // command pool
            let command_pool_create_info = vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: suitable_device.graphics_queue_family,
            };
            let command_pool = unsafe { device.create_command_pool(&command_pool_create_info, None) }?;

            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: ptr::null(),
                command_pool,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
            };

            let command_buffers = unsafe {device.allocate_command_buffers(&command_buffer_allocate_info)}?;
            let primary_command_buffer = command_buffers
                .into_iter()
                .next()
                .into_ris_error()?;

            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: ptr::null(),
                command_pool,
                level: vk::CommandBufferLevel::SECONDARY,
                command_buffer_count: command_buffer_count as u32,
            };
            let secondary_command_buffers = unsafe {device.allocate_command_buffers(&command_buffer_allocate_info)}?;

            // frame buffers
            let framebuffers = vec![None; renderer_count];

            // synchronization
            let semaphore_create_info = vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::SemaphoreCreateFlags::empty(),
            };
            let fence_create_info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::FenceCreateFlags::SIGNALED,
            };

            let image_available = unsafe {device.create_semaphore(&semaphore_create_info, None)}?;

            let mut renderer_finished = Vec::with_capacity(renderer_count);
            for _ in 0..renderer_finished.capacity() {
                let semaphore = unsafe {device.create_semaphore(&semaphore_create_info, None)}?;
                renderer_finished.push(semaphore);
            }

            let command_buffer_finished = unsafe {device.create_semaphore(&semaphore_create_info, None)}?;

            let done = unsafe {device.create_fence(&fence_create_info, None)}?;

            // construct frame
            let entry = FrameInFlight {
                index: i,
                command_pool,
                primary_command_buffer,
                secondary_command_buffers,
                framebuffers,
                image_available,
                renderer_finished,
                command_buffer_finished,
                done,
            };
            entries.push(entry);
        }

        Ok(Self {
            entries,
            current_frame: 0,
        })
    }

    pub fn acquire_next_frame(&mut self, device: &ash::Device) -> RisResult<&mut FrameInFlight> {
        let entry = &mut self.entries[self.current_frame];

        unsafe {
            let fences = [entry.done];

            device.wait_for_fences(&fences, true, u64::MAX)?;
            device.reset_fences(&fences)?;
        }

        Ok(entry)
    }
}

impl FrameInFlight {
    pub fn command_buffers(&self, id: &RendererId) -> &[vk::CommandBuffer] {
        let start = id.command_buffers_start;
        let end = id.command_buffers_end;
        &self.secondary_command_buffers[start..end]
    }

    pub fn frame_buffer(
        &mut self,
        id: &RendererId,
        device: &ash::Device,
        framebuffer_create_info: vk::FramebufferCreateInfo,
    ) -> RisResult<vk::Framebuffer> {
        let index = id.index;

        let framebuffer = self.framebuffers.get_mut(index).into_ris_error()?;

        match framebuffer {
            Some(framebuffer) => Ok(*framebuffer),
            None => {
                let new_framebuffer = unsafe {device.create_framebuffer(&framebuffer_create_info, None)}?;
                *framebuffer = Some(new_framebuffer);
                Ok(new_framebuffer)
            }
        }
    }

    pub fn semaphore(&self, id: &RendererId) -> vk::Semaphore {
        let index = id.index as isize;
        self.semaphore_internal(index)
    }

    pub fn previous_semaphore(&self, id: &RendererId) -> vk::Semaphore {
        let index = id.index as isize - 1;
        self.semaphore_internal(index)
    }

    fn semaphore_internal(&self, index: isize) -> vk::Semaphore {
        if index < 0 {
            self.image_available
        } else {
            self.renderer_finished[index as usize]
        }
    }
}
