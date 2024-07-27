use std::ptr;

use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

use super::transient_command::TransientCommand;
use super::transient_command::TransientCommandSync;
use super::util;

pub struct Image {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
}

pub struct ImageCreateInfo<'a> {
    pub device: &'a ash::Device,
    pub width: u32,
    pub height: u32,
    pub format: vk::Format,
    pub tiling: vk::ImageTiling,
    pub usage: vk::ImageUsageFlags,
    pub memory_property_flags: vk::MemoryPropertyFlags,
    pub physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
}

pub struct TransitionLayoutInfo<'a> {
    pub device: &'a ash::Device,
    pub queue: vk::Queue,
    pub transient_command_pool: vk::CommandPool,
    pub format: vk::Format,
    pub old_layout: vk::ImageLayout,
    pub new_layout: vk::ImageLayout,
    pub sync: TransientCommandSync,
}

impl Image {
    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(info: ImageCreateInfo) -> RisResult<Self> {
        let ImageCreateInfo {
            device,
            width,
            height,
            format,
            tiling,
            usage,
            memory_property_flags,
            physical_device_memory_properties,
        } = info;

        let image_create_info = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ImageCreateFlags::empty(),
            image_type: vk::ImageType::TYPE_2D,
            format,
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling,
            usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
            initial_layout: vk::ImageLayout::UNDEFINED,
        };

        let image = unsafe { device.create_image(&image_create_info, None) }?;

        let image_memory_requirements = unsafe { device.get_image_memory_requirements(image) };
        let memory_type_index = super::util::find_memory_type(
            image_memory_requirements.memory_type_bits,
            memory_property_flags,
            physical_device_memory_properties,
        )?
        .unroll()?;

        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: image_memory_requirements.size,
            memory_type_index,
        };

        let memory = unsafe { device.allocate_memory(&memory_allocate_info, None) }?;
        unsafe { device.bind_image_memory(image, memory, 0) }?;

        Ok(Self { image, memory })
    }

    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_image(self.image, None);
            device.free_memory(self.memory, None);
        }
    }

    /// # Safety
    ///
    /// the resulting `ImageView` must be explicitly destroyed.
    pub unsafe fn alloc_view(
        device: &ash::Device,
        image: vk::Image,
        format: vk::Format,
        aspect_mask: vk::ImageAspectFlags,
    ) -> RisResult<vk::ImageView> {
        let image_view_create_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image,
            view_type: vk::ImageViewType::TYPE_2D,
            format,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::IDENTITY,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::IDENTITY,
                a: vk::ComponentSwizzle::IDENTITY,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        let view = unsafe { device.create_image_view(&image_view_create_info, None) }?;

        Ok(view)
    }

    pub fn transition_layout(&self, info: TransitionLayoutInfo) -> RisResult<()> {
        let TransitionLayoutInfo {
            device,
            queue,
            transient_command_pool,
            format,
            old_layout,
            new_layout,
            sync,
        } = info;

        let transient_command = TransientCommand::begin(device, queue, transient_command_pool)?;

        let aspect_mask = if new_layout == vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL {
            let mut aspect_mask = vk::ImageAspectFlags::DEPTH;

            if util::has_stencil_component(format) {
                aspect_mask |= vk::ImageAspectFlags::STENCIL;
            }

            aspect_mask
        } else {
            vk::ImageAspectFlags::COLOR
        };

        let (src_access_mask, dst_access_mask, stc_stage_mask, dst_stage_mask) =
            match (old_layout, new_layout) {
                (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
                    vk::AccessFlags::empty(),
                    vk::AccessFlags::TRANSFER_WRITE,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::TRANSFER,
                ),
                (
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                ) => (
                    vk::AccessFlags::TRANSFER_WRITE,
                    vk::AccessFlags::SHADER_READ,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                ),
                (vk::ImageLayout::UNDEFINED, vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL) => {
                    (
                        vk::AccessFlags::empty(),
                        vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                        vk::PipelineStageFlags::TOP_OF_PIPE,
                        vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
                    )
                }
                transition => {
                    return ris_error::new_result!(
                        "unsupported transition from {:?} to {:?}",
                        transition.0,
                        transition.1,
                    )
                }
            };

        let image_memory_barriers = [vk::ImageMemoryBarrier {
            s_type: vk::StructureType::IMAGE_MEMORY_BARRIER,
            p_next: ptr::null(),
            src_access_mask,
            dst_access_mask,
            old_layout,
            new_layout,
            src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            image: self.image,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        }];

        unsafe {
            device.cmd_pipeline_barrier(
                transient_command.buffer(),
                stc_stage_mask,
                dst_stage_mask,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &image_memory_barriers,
            )
        };

        transient_command.end_and_submit(sync)?;
        Ok(())
    }
}
