use std::ptr;

use ash::vk;

use ris_asset::RisGodAsset;
use ris_error::RisResult;
use ris_math::camera::Camera;
use ris_math::matrix::Mat4;
use ris_video_data::buffer::Buffer;
use ris_video_data::core::VulkanCore;
use ris_video_data::swapchain::SwapchainEntry;
use ris_video_data::texture::Texture;
use ris_video_data::texture::TextureCreateInfo;

use crate::scene::scene_renderer::SceneFrame;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UniformBufferObject {
    pub view: Mat4,
    pub proj: Mat4,
}

pub struct TerrainFrame {
    framebuffer: Option<vk::Framebuffer>,
    descriptor_buffer: Buffer,
    descriptor_mapped: *mut UniformBufferObject,
    descriptor_set: vk::DescriptorSet,
}

impl TerrainFrame {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        if let Some(framebuffer) = self.framebuffer.take() {
            device.destroy_framebuffer(framebuffer, None);
        }

        self.descriptor_buffer.free(device);
    }
}

pub struct TerrainRenderer {
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    frames: Vec<TerrainFrame>,
}

impl TerrainRenderer {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        unsafe {
            for frame in self.frames.iter_mut() {
                frame.free(device);
            }

            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);
        }
    }

    pub fn alloc(
        core: &VulkanCore,
        god_asset: &RisGodAsset,
    ) -> RisResult<Self> {
        let VulkanCore {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            swapchain,
            ..
        } = core;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };
        let physical_device_properties =
            unsafe { instance.get_physical_device_properties(suitable_device.physical_device) };

        // descriptor sets
        let descriptor_set_layout_bindings = [
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::VERTEX,
                p_immutable_samplers: ptr::null(),
            },
        ];

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: descriptor_set_layout_bindings.len() as u32,
            p_bindings: descriptor_set_layout_bindings.as_ptr(),
        };

        let descriptor_set_layout = unsafe {
            device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
        }?;

        let descriptor_pool_sizes = [
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: swapchain.entries.len() as u32,
            },
        ];

        let total_descriptor_set_count = swapchain.entries.len();
        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorPoolCreateFlags::empty(),
            max_sets: total_descriptor_set_count as u32,
            pool_size_count: descriptor_pool_sizes.len() as u32,
            p_pool_sizes: descriptor_pool_sizes.as_ptr(),
        };

        let descriptor_pool = unsafe{
            device.create_descriptor_pool(&descriptor_pool_create_info, None)
        }?;

        let mut descriptor_set_layouts = Vec::with_capacity(total_descriptor_set_count);
        for _ in 0..total_descriptor_set_count {
            descriptor_set_layouts.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool,
            descriptor_set_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
        };

        let descriptor_sets = unsafe {
            device.allocate_descriptor_sets(&descriptor_set_allocate_info)
        }?;

        // shaders

        ris_error::new_result!("reached end")
    }

    pub fn draw(
        &mut self,
    ) -> RisResult<()> {
        ris_log::info!("render terrain");
        Ok(())
    }
}
