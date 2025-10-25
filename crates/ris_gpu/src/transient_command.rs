use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

#[derive(Clone)]
pub struct TransientCommandArgs {
    pub device: ash::Device,
    pub queue: vk::Queue,
    pub command_pool: vk::CommandPool,
}

pub struct TransientCommand {
    device: ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
}

impl Drop for TransientCommand {
    fn drop(&mut self) {
        unsafe { self.free() };
    }
}

impl TransientCommand {
    unsafe fn free(&mut self) {
        let device = &self.device;
        let command_pool = self.command_pool;
        let command_buffer = self.command_buffer;

        device.free_command_buffers(command_pool, &[command_buffer]);
    }

    /// # Safety
    ///
    /// vulkan objects passed into this function must outlive the
    /// transient command
    pub unsafe fn begin(args: TransientCommandArgs) -> RisResult<Self> {
        let device = args.device.clone();
        let queue = args.queue;
        let command_pool = args.command_pool;

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            command_buffer_count: 1,
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
        };

        let command_buffers =
            unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }?;
        let command_buffer = *command_buffers.first().into_ris_error()?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: std::ptr::null(),
        };

        unsafe { device.begin_command_buffer(command_buffer, &command_buffer_begin_info) }?;

        Ok(Self {
            device,
            queue,
            command_pool,
            command_buffer,
        })
    }

    pub fn buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    pub fn submit_and_wait(self, fence: Option<vk::Fence>) -> RisResult<()> {
        let Self {
            device,
            queue,
            command_buffer,
            ..
        } = &self;

        let submit_info = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: 0,
            p_wait_semaphores: std::ptr::null(),
            p_wait_dst_stage_mask: std::ptr::null(),
            command_buffer_count: 1,
            p_command_buffers: command_buffer,
            signal_semaphore_count: 0,
            p_signal_semaphores: std::ptr::null(),
        }];

        let (fence, destroy_fence) = match fence {
            Some(fence) => (fence, false),
            None => {
                let fence_create_info = vk::FenceCreateInfo {
                    s_type: vk::StructureType::FENCE_CREATE_INFO,
                    p_next: std::ptr::null(),
                    flags: vk::FenceCreateFlags::empty(),
                };

                let fence = unsafe { device.create_fence(&fence_create_info, None) }?;
                (fence, true)
            }
        };

        unsafe {
            device.end_command_buffer(self.buffer())?;
            device.queue_submit(*queue, &submit_info, fence)?;
            device.wait_for_fences(&[fence], true, u64::MAX)?;
            if destroy_fence {
                device.destroy_fence(fence, None);
            }
        };

        Ok(())
    }
}
