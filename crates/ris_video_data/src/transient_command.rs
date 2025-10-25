use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

pub mod prelude {
    pub use super::TransientCommand;
    pub use super::TransientCommandArgs;
    pub use super::TransientCommandSync;
}

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

#[derive(Debug, Clone)]
pub struct TransientCommandSync {
    pub wait: Vec<vk::Semaphore>,
    pub dst: Vec<vk::PipelineStageFlags>,
    pub signal: Vec<vk::Semaphore>,
    pub fence: vk::Fence,
}

impl TransientCommand {
    unsafe fn free(&mut self) {
        let device = self.device.clone();
        let command_pool = self.command_pool;
        let command_buffer = self.command_buffer;

        device.free_command_buffers(command_pool, &[command_buffer]);
    }

    /// # Safety
    ///
    /// vulkan objects passed into this function must outlive the
    /// transient command. note that dropping the transient command spawns
    /// a job that waits till it is fully executed. this means the vulkan
    /// objects must live even beyond that.
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

    pub unsafe fn end_and_submit(mut self, sync: TransientCommandSync) -> RisResult<()> {
        ris_error::debug_assert!(sync.wait.len() == sync.dst.len())?;

        let Self {
            device,
            queue,
            command_buffer,
            ..
        } = &self;

        let submit_info = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: sync.wait.len() as u32,
            p_wait_semaphores: sync.wait.as_ptr(),
            p_wait_dst_stage_mask: sync.dst.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: command_buffer,
            signal_semaphore_count: sync.signal.len() as u32,
            p_signal_semaphores: sync.signal.as_ptr(),
        }];

        unsafe {
            device.end_command_buffer(self.buffer())?;
            device.queue_submit(*queue, &submit_info, sync.fence)?;
        };

        Ok(())
    }
}

