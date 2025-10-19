use ash::vk;

use ris_async::JobFuture;
use ris_error::Extensions;
use ris_error::RisResult;

use super::transient_command::prelude::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
pub struct Image {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
    format: vk::Format,
    layout: vk::ImageLayout,
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

pub struct TransitionLayoutInfo {
    pub transient_command_args: TransientCommandArgs,
    pub new_layout: vk::ImageLayout,
    pub sync: TransientCommandSync,
}

impl Image {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_image(self.image, None);
            device.free_memory(self.memory, None);
        }
    }

    pub fn alloc(info: ImageCreateInfo) -> RisResult<Self> {
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

        let layout = vk::ImageLayout::UNDEFINED;

        let image_create_info = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: std::ptr::null(),
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
            p_queue_family_indices: std::ptr::null(),
            initial_layout: layout,
        };

        let image = unsafe { device.create_image(&image_create_info, None) }?;

        let image_memory_requirements = unsafe { device.get_image_memory_requirements(image) };
        let memory_type_index = super::util::find_memory_type(
            image_memory_requirements.memory_type_bits,
            memory_property_flags,
            physical_device_memory_properties,
        )?
        .into_ris_error()?;

        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            allocation_size: image_memory_requirements.size,
            memory_type_index,
        };

        let memory = unsafe { device.allocate_memory(&memory_allocate_info, None) }?;
        unsafe { device.bind_image_memory(image, memory, 0) }?;

        Ok(Self { 
            image,
            memory,
            format,
            layout,
        })
    }

    pub fn alloc_view(
        device: &ash::Device,
        image: vk::Image,
        format: vk::Format,
        aspect_mask: vk::ImageAspectFlags,
    ) -> RisResult<vk::ImageView> {
        let image_view_create_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: std::ptr::null(),
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

    pub fn transition_layout(&mut self, info: TransitionLayoutInfo) -> RisResult<JobFuture<()>> {
        let TransitionLayoutInfo {
            transient_command_args,
            new_layout,
            sync,
        } = info;

        let aspect_mask = if new_layout == vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL {
            let mut aspect_mask = vk::ImageAspectFlags::DEPTH;

            if super::util::has_stencil_component(self.format) {
                aspect_mask |= vk::ImageAspectFlags::STENCIL;
            }

            aspect_mask
        } else {
            vk::ImageAspectFlags::COLOR
        };

        struct Mask {
            src_access: vk::AccessFlags,
            dst_access: vk::AccessFlags,
            src_pipeline_stage: vk::PipelineStageFlags,
            dst_pipeline_stage: vk::PipelineStageFlags,
        }

        let mask =
            match (self.layout, new_layout) {
                (
                    vk::ImageLayout::UNDEFINED,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                ) => Mask {
                    src_access: vk::AccessFlags::empty(),
                    dst_access: vk::AccessFlags::TRANSFER_WRITE,
                    src_pipeline_stage: vk::PipelineStageFlags::TOP_OF_PIPE,
                    dst_pipeline_stage: vk::PipelineStageFlags::TRANSFER,
                },
                (
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                ) => Mask {
                    src_access: vk::AccessFlags::TRANSFER_WRITE,
                    dst_access: vk::AccessFlags::SHADER_READ,
                    src_pipeline_stage: vk::PipelineStageFlags::TRANSFER,
                    dst_pipeline_stage: vk::PipelineStageFlags::FRAGMENT_SHADER,
                },
                (
                    vk::ImageLayout::UNDEFINED,
                    vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                ) => Mask {
                    src_access: vk::AccessFlags::empty(),
                    dst_access: vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                        | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                    src_pipeline_stage: vk::PipelineStageFlags::TOP_OF_PIPE,
                    dst_pipeline_stage: vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
                },
                transition => {
                    return ris_error::new_result!(
                        "TODO: transition from {:?} to {:?} is not yet implemented",
                        transition.0,
                        transition.1,
                    )
                }
            };

        let image_memory_barriers = [vk::ImageMemoryBarrier {
            s_type: vk::StructureType::IMAGE_MEMORY_BARRIER,
            p_next: std::ptr::null(),
            src_access_mask: mask.src_access,
            dst_access_mask: mask.dst_access,
            old_layout: self.layout,
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

        let device = transient_command_args.device.clone();

        let future = unsafe {
            let transient_command = TransientCommand::begin(transient_command_args)?;

            device.cmd_pipeline_barrier(
                transient_command.buffer(),
                mask.src_pipeline_stage,
                mask.dst_pipeline_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &image_memory_barriers,
            );

            transient_command.end_and_submit(sync)?
        };

        self.layout = new_layout;

        Ok(future)
    }
}
