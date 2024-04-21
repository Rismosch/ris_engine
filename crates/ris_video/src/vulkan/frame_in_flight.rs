use ash::vk;

use super::buffer::Buffer;
use super::uniform_buffer_object::UniformBufferObject;

pub struct FrameInFlight {
    pub command_buffer: vk::CommandBuffer,
    pub uniform_buffer: Buffer,
    pub uniform_buffer_mapped: *mut UniformBufferObject,
    pub descriptor_set: vk::DescriptorSet,
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore,
    pub in_flight_fence: vk::Fence,
}
