use ash::vk;

use ris_error::RisResult;

use super::buffer::Buffer;
use super::image::Image;
use super::image::ImageCreateInfo;
use super::image::TransitionLayoutInfo;
use super::transient_command::TransientCommandArgs;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
pub struct Texture {
    pub image: Image,
    pub view: vk::ImageView,
    pub sampler: vk::Sampler,
}

pub struct TextureCreateInfo<'a> {
    pub transient_command_args: TransientCommandArgs,
    pub staging: &'a Buffer,
    pub physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub width: usize,
    pub height: usize,
    pub format: vk::Format,
    pub filter: vk::Filter,
    pub pixels: &'a [u8],
}

impl Texture {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&self, device: &ash::Device) {
        device.destroy_sampler(self.sampler, None);
        device.destroy_image_view(self.view, None);
        self.image.free(device);
    }

    pub fn alloc(info: TextureCreateInfo) -> RisResult<Self> {
        let TextureCreateInfo {
            transient_command_args,
            staging,
            physical_device_memory_properties,
            physical_device_properties,
            width,
            height,
            format,
            filter,
            pixels,
        } = info;

        ris_error::assert!(width != 0)?;
        ris_error::assert!(height != 0)?;

        // https://github.com/Rismosch/ris_engine/issues/209
        let pixel_width = 4;

        let actual_len = pixels.len();
        let expected_len = width * height * pixel_width;
        ris_error::assert!(actual_len == expected_len)?;

        let device = transient_command_args.device.clone();
        let tcas = transient_command_args.clone();

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::FenceCreateFlags::empty(),
        };
        let fence = unsafe { device.create_fence(&fence_create_info, None) }?;

        // create image and copy asset to it
        let mut image = Image::alloc(ImageCreateInfo {
            device: device.clone(),
            width,
            height,
            format,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            physical_device_memory_properties,
        })?;

        image.transition_layout(TransitionLayoutInfo {
            transient_command_args: tcas.clone(),
            new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            fence: Some(fence),
        })?;

        unsafe {
            device.reset_fences(&[fence])?;

            super::io::write_to_image(super::io::Args {
                transient_command_args: tcas.clone(),
                values: pixels,
                gpu_object: &image,
                staging,
            })?;
        }

        image.transition_layout(TransitionLayoutInfo {
            transient_command_args: tcas.clone(),
            new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            fence: Some(fence),
        })?;

        unsafe {
            device.destroy_fence(fence, None);
        }

        // create image view
        let view = Image::alloc_view(
            device.clone(),
            image.image,
            format,
            vk::ImageAspectFlags::COLOR,
        )?;

        // create sampler
        let sampler_create_info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SAMPLER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::SamplerCreateFlags::empty(),
            mag_filter: filter,
            min_filter: filter,
            mipmap_mode: vk::SamplerMipmapMode::NEAREST,
            address_mode_u: vk::SamplerAddressMode::REPEAT,
            address_mode_v: vk::SamplerAddressMode::REPEAT,
            address_mode_w: vk::SamplerAddressMode::REPEAT,
            mip_lod_bias: 0.0,
            anisotropy_enable: vk::TRUE,
            max_anisotropy: physical_device_properties.limits.max_sampler_anisotropy,
            compare_enable: vk::FALSE,
            compare_op: vk::CompareOp::ALWAYS,
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalized_coordinates: vk::FALSE,
        };

        let sampler = unsafe { device.create_sampler(&sampler_create_info, None) }?;

        Ok(Self {
            image,
            view,
            sampler,
        })
    }
}
