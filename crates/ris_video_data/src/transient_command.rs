use std::ptr;

use ash::vk;

use ris_async::JobFuture;
use ris_async::ThreadPool;
use ris_error::Extensions;
use ris_error::RisResult;

#[derive(Default, Debug)]
pub struct TransientCommandSync {
    pub wait: Vec<vk::Semaphore>,
    pub dst: Vec<vk::PipelineStageFlags>,
    pub signal: Vec<vk::Semaphore>,
}

pub struct TransientCommand {
    device: ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    fence: vk::Fence,
    free_on_drop: bool,
}

impl Drop for TransientCommand {
    fn drop(&mut self) {
        if self.free_on_drop {
            unsafe {self.free()};
        }

    }
}

impl TransientCommand {
    unsafe fn free(&mut self) {
        let device = self.device.clone();
        let command_pool = self.command_pool;
        let command_buffer = self.command_buffer;
        let fence = self.fence;

        device.destroy_fence(fence, None);
        device.free_command_buffers(command_pool, &[command_buffer]);
    }

    /// # Safety
    ///
    /// vulkan objects passed into this function must outlive the
    /// transient command. note that dropping the transient command spawns
    /// a job that waits till it is fully executed. this means the vulkan
    /// objects must live even beyond that.
    pub unsafe fn begin(
        device: &ash::Device,
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
        let command_buffer = *command_buffers.first().into_ris_error()?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: ptr::null(),
        };

        unsafe { device.begin_command_buffer(command_buffer, &command_buffer_begin_info) }?;

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };
        let fence = unsafe {device.create_fence(&fence_create_info, None)}?;

        Ok(Self {
            device: device.clone(),
            queue,
            command_pool,
            command_buffer,
            fence,
            free_on_drop: true,
        })
    }

    pub fn buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    /// # Safety
    ///
    /// any vulkan objects passed into the transient command, either by
    /// calling `begin()` or by enqueuing commands into the command
    /// buffer, retreived by `buffer()`, must outlive a significant time
    /// after submitting it. this is because execution is async and all
    /// references must stay valid during execution. to help with
    /// synchronization, this function returns a future that you can wait
    /// on before destroying any resource.
    pub unsafe fn end_and_submit(mut self, sync: TransientCommandSync) -> RisResult<JobFuture<()>> {
        ris_error::debug_assert!(sync.wait.len() == sync.dst.len())?;

        self.free_on_drop = false;

        let Self {
            device,
            queue,
            command_buffer,
            ..
        } = &self;

        let submit_info = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
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
            device.reset_fences(&[self.fence])?;
            device.queue_submit(*queue, &submit_info, self.fence)?;
        };

        let device = self.device.clone();
        let future = ThreadPool::submit(async move {
            block_on_fence(&device, self.fence);
            self.free();
        });

        Ok(future)
    }
}

fn block_on_fence(device: &ash::Device, fence: vk::Fence) {
    let fences = [fence];

    loop {
        let result = unsafe {device.wait_for_fences(&fences, true, 0)};
        if result.is_ok() {
            break;
        }

        if !ThreadPool::run_pending_job() {
            std::hint::spin_loop();
        }
    }
}
