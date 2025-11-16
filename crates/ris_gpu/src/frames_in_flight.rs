use ash::vk;

use ris_error::prelude::*;

use super::debug::Debugger;
use super::suitable_device::SuitableDevice;

// with no frames in flight (FRAMES_IN_FLIGHT == 1), we always
// need to wait for the command buffers to be done executing
// before recording them again. recording may be expensive,
// thus leaving the GPU to run idle. to maximize hardware usage,
// we allow recording of newcommand buffers, while previous ones
// are being executed. higher values for MAX_FRAMES_IN_FLIGHT
// allow more parallelism, but also more frames produce more
// latency, as the GPU may lag behind the CPU.
//
// no frames in flight provide the lowest latency
pub const FRAMES_IN_FLIGHT: usize = 2;
const _: () = assert!(FRAMES_IN_FLIGHT > 0, "FRAMES_IN_FLIGHT may not be 0");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RendererId {
    index: usize,
    secondary_command_buffers_start: usize,
    secondary_command_buffers_end: usize,
}

impl RendererId {
    pub fn index(&self) -> usize {
        self.index
    }
}

pub struct FramesInFlight {
    pub entries: Vec<FrameInFlight>,
    pub current_frame: usize,
}

pub struct FrameInFlight {
    pub index: usize,

    pub command_pool: vk::CommandPool,
    pub primary_command_buffers: Vec<vk::CommandBuffer>, // one per renderer
    pub secondary_command_buffers: Vec<vk::CommandBuffer>, // none or multiple per renderer

    pub image_available: vk::Semaphore,
    pub finished_fence: vk::Fence,
}

pub struct RendererRegisterer<'a> {
    pub info: FrameInFlightCreateInfo<'a>,
    pub existing_id: Option<RendererId>,
}

impl<'a> RendererRegisterer<'a> {
    pub fn register(&mut self, secondary_command_buffer_count: usize) -> RisResult<RendererId> {
        let id = match self.existing_id {
            Some(id) => {
                let start = id.secondary_command_buffers_start;
                let end = id.secondary_command_buffers_end;
                let count = end - start;
                ris_error::assert!(count == secondary_command_buffer_count)?;
                id
            }
            None => self.info.register_renderer(secondary_command_buffer_count),
        };

        Ok(id)
    }
}

pub struct FrameInFlightCreateInfo<'a> {
    pub debugger: &'a Debugger,
    pub suitable_device: &'a SuitableDevice,
    pub device: &'a ash::Device,
    pub renderer_count: usize,
    pub secondary_command_buffer_count: usize,
}

impl<'a> FrameInFlightCreateInfo<'a> {
    fn register_renderer(&mut self, secondary_command_buffer_count: usize) -> RendererId {
        let index = self.renderer_count;
        let secondary_command_buffers_start = self.secondary_command_buffer_count;
        let secondary_command_buffers_end =
            secondary_command_buffers_start + secondary_command_buffer_count;

        self.renderer_count += 1;
        self.secondary_command_buffer_count += secondary_command_buffer_count;

        RendererId {
            index,
            secondary_command_buffers_start,
            secondary_command_buffers_end,
        }
    }
}

impl FramesInFlight {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            // free synchronization
            device.destroy_fence(entry.finished_fence, None);
            device.destroy_semaphore(entry.image_available, None);

            // free command buffers
            if !entry.secondary_command_buffers.is_empty() {
                device.free_command_buffers(entry.command_pool, &entry.secondary_command_buffers);
            }
            device.free_command_buffers(entry.command_pool, &entry.primary_command_buffers);
            device.destroy_command_pool(entry.command_pool, None);
        }

        self.entries.clear();
    }

    pub fn alloc(info: FrameInFlightCreateInfo) -> RisResult<Self> {
        let FrameInFlightCreateInfo {
            debugger,
            suitable_device,
            device,
            renderer_count,
            secondary_command_buffer_count,
        } = info;

        let mut entries = Vec::with_capacity(FRAMES_IN_FLIGHT);
        for i in 0..entries.capacity() {
            // command pool
            let command_pool_create_info = vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: suitable_device.graphics_queue_family,
            };
            let command_pool =
                unsafe { device.create_command_pool(&command_pool_create_info, None) }?;
            debugger.set_name(
                device,
                command_pool,
                format!("frame_in_flight_{}_command_pool", i),
            )?;

            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                command_pool,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: renderer_count as u32,
            };

            let primary_command_buffers =
                unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }?;
            for (j, primary_command_buffer) in primary_command_buffers.iter().enumerate() {
                debugger.set_name(
                    device,
                    *primary_command_buffer,
                    format!("frame_in_flight_{}_primary_command_buffer_{}", i, j),
                )?;
            }

            let secondary_command_buffers = if secondary_command_buffer_count == 0 {
                Vec::with_capacity(0)
            } else {
                let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                    s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                    p_next: std::ptr::null(),
                    command_pool,
                    level: vk::CommandBufferLevel::SECONDARY,
                    command_buffer_count: secondary_command_buffer_count as u32,
                };
                unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }?
            };
            for (j, secondary_command_buffer) in secondary_command_buffers.iter().enumerate() {
                debugger.set_name(
                    device,
                    *secondary_command_buffer,
                    format!("frame_in_flight_{}_secondary_command_buffer_{}", i, j),
                )?;
            }

            // synchronization
            let semaphore_create_info = vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::SemaphoreCreateFlags::empty(),
            };
            let fence_create_info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::FenceCreateFlags::SIGNALED,
            };

            let image_available = unsafe { device.create_semaphore(&semaphore_create_info, None) }?;
            let finished_fence = unsafe { device.create_fence(&fence_create_info, None) }?;
            debugger.set_name(
                device,
                image_available,
                format!("frame_in_flight_{}_image_available", i),
            )?;
            debugger.set_name(
                device,
                finished_fence,
                format!("frame_in_flight_{}_finished_fence", i),
            )?;

            // construct frame
            let entry = FrameInFlight {
                index: i,
                command_pool,
                primary_command_buffers,
                secondary_command_buffers,
                image_available,
                finished_fence,
            };
            entries.push(entry);
        }

        Ok(Self {
            entries,
            current_frame: 0,
        })
    }

    pub fn acquire_next_frame(&mut self, device: &ash::Device) -> RisResult<&FrameInFlight> {
        let previous_frame = self.current_frame;
        let next_frame = (self.current_frame + 1) % self.entries.len();
        self.current_frame = next_frame;

        let entry = &self.entries[previous_frame];

        unsafe {
            let fences = [entry.finished_fence];
            device.wait_for_fences(&fences, true, u64::MAX)?;
            device.reset_fences(&fences)?;
        }

        Ok(entry)
    }
}

impl FrameInFlight {
    pub fn primary_command_buffer(&self, id: RendererId) -> vk::CommandBuffer {
        let index = id.index();
        self.primary_command_buffers[index]
    }

    pub fn secondary_command_buffers(&self, id: RendererId) -> &[vk::CommandBuffer] {
        let start = id.secondary_command_buffers_start;
        let end = id.secondary_command_buffers_end;
        &self.secondary_command_buffers[start..end]
    }
}
