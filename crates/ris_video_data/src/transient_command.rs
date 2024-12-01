use std::ptr;

use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

#[derive(Default, Debug)]
pub struct TransientCommandSync {
    pub wait: Vec<vk::Semaphore>,
    pub dst: Vec<vk::PipelineStageFlags>,
    pub signal: Vec<vk::Semaphore>,
    pub fence: vk::Fence,
}

impl TransientCommandSync {
    pub fn sync_now(self, device: &ash::Device, queue: vk::Queue) -> RisResult<()> {
        queue_submit(device, queue, &[], self)
    }
}

pub struct TransientCommand<'a> {
    device: &'a ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
}

impl<'a> Drop for TransientCommand<'a> {
    fn drop(&mut self) {
        let Self {
            device,
            queue,
            command_pool,
            command_buffers,
        } = self;

        unsafe {
            ris_error::unwrap!(device.queue_wait_idle(*queue), "failed to queue wait idle",);

            device.free_command_buffers(*command_pool, command_buffers);
        }
    }
}

impl<'a> TransientCommand<'a> {
    pub fn begin(
        device: &'a ash::Device,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
    ) -> RisResult<Self> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_buffer_count: 1,
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
        };

        let command_buffers =
            unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }?;
        let command_buffer = command_buffers.first().into_ris_error()?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: ptr::null(),
        };

        unsafe { device.begin_command_buffer(*command_buffer, &command_buffer_begin_info) }?;

        Ok(Self {
            device,
            queue,
            command_pool,
            command_buffers,
        })
    }

    pub fn buffer(&self) -> vk::CommandBuffer {
        // cannot cause ub, because `begin()` would've failed if no command buffer exists
        *unsafe { self.command_buffers.get_unchecked(0) }
    }

    pub fn end_and_submit(self, sync: TransientCommandSync) -> RisResult<()> {
        ris_error::debug_assert!(sync.wait.len() == sync.dst.len())?;

        let Self {
            device,
            queue,
            command_buffers,
            ..
        } = &self;

        unsafe { device.end_command_buffer(self.buffer()) }?;

        queue_submit(device, *queue, command_buffers, sync)
    }
}

fn queue_submit(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffers: &[vk::CommandBuffer],
    sync: TransientCommandSync,
) -> RisResult<()> {
    let submit_info = [vk::SubmitInfo {
        s_type: vk::StructureType::SUBMIT_INFO,
        p_next: ptr::null(),
        wait_semaphore_count: sync.wait.len() as u32,
        p_wait_semaphores: sync.wait.as_ptr(),
        p_wait_dst_stage_mask: sync.dst.as_ptr(),
        command_buffer_count: command_buffers.len() as u32,
        p_command_buffers: command_buffers.as_ptr(),
        signal_semaphore_count: sync.signal.len() as u32,
        p_signal_semaphores: sync.signal.as_ptr(),
    }];

    unsafe { device.queue_submit(queue, &submit_info, sync.fence) }?;

    Ok(())
}
