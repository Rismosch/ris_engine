use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

pub struct TransientCommand<'a> {
    device: &'a ash::Device,
    queue: &'a vk::Queue,
    command_pool: &'a vk::CommandPool,
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
            ris_error::unwrap!(
                device.queue_wait_idle(**queue),
                "failed to queue wait idle",
            );

            device.free_command_buffers(**command_pool, command_buffers);
        }
    }
}

impl<'a> TransientCommand<'a> {
    pub fn begin(
        device: &'a ash::Device,
        queue: &'a vk::Queue,
        command_pool: &'a vk::CommandPool) -> RisResult<Self> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_buffer_count: 1,
            command_pool: *command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
        };

        let command_buffers = unsafe {device.allocate_command_buffers(&command_buffer_allocate_info)}?;
        let command_buffer = command_buffers.first().unroll()?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: ptr::null(),
        };

        unsafe{device.begin_command_buffer(*command_buffer, &command_buffer_begin_info)}?;

        Ok(Self{
            device,
            queue,
            command_pool,
            command_buffers,
        })
    }

    pub fn buffer(&self) -> &vk::CommandBuffer {
        // cannot cause ub, because `begin(1)` would've failed if no command buffer exists
        unsafe{self.command_buffers.get_unchecked(0)}
    }

    pub fn end_and_submit(self) -> RisResult<()> {
        let Self {
            device,
            queue,
            command_buffers,
            ..
        } = &self;

        unsafe{device.end_command_buffer(*self.buffer())}?;

        let submit_info = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 0,
            p_wait_semaphores: ptr::null(),
            p_wait_dst_stage_mask: ptr::null(),
            command_buffer_count: command_buffers.len() as u32,
            p_command_buffers: command_buffers.as_ptr(),
            signal_semaphore_count: 0,
            p_signal_semaphores: ptr::null(),
        }];

        unsafe {device.queue_submit(**queue, &submit_info, vk::Fence::null())}?;

        Ok(())
    }
}
