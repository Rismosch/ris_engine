use std::ptr;

use ash::vk;

use ris_error::RisResult;

use super::buffer::Buffer;
use super::buffer::CopyToImageInfo;
use super::image::Image;
use super::image::ImageCreateInfo;
use super::image::TransitionLayoutInfo;
use super::transient_command::TransientCommandSync;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
pub struct Texture {
    pub image: Image,
    pub view: vk::ImageView,
    pub sampler: vk::Sampler,
}

pub struct TextureCreateInfo<'a> {
    pub device: &'a ash::Device,
    pub queue: vk::Queue,
    pub transient_command_pool: vk::CommandPool,
    pub physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub width: u32,
    pub height: u32,
    pub format: vk::Format,
    pub filter: vk::Filter,
    pub pixels_rgba: &'a [u8],
}

impl Texture {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&self, device: &ash::Device) {
        device.destroy_sampler(self.sampler, None);
        device.destroy_image_view(self.view, None);

        self.image.free(device);
    }

    pub fn alloc(info: TextureCreateInfo) -> RisResult<Self> {
        let TextureCreateInfo {
            device,
            queue,
            transient_command_pool,
            physical_device_memory_properties,
            physical_device_properties,
            width,
            height,
            format,
            filter,
            pixels_rgba,
        } = info;

        ris_error::debug_assert!(width != 0)?;
        ris_error::debug_assert!(height != 0)?;

        let actual_len = pixels_rgba.len();
        let expected_len = (width * height * 4) as usize;
        ris_error::debug_assert!(actual_len == expected_len)?;

        // create image and copy asset to it
        let staging_buffer = Buffer::alloc(
            device,
            pixels_rgba.len() as vk::DeviceSize,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            physical_device_memory_properties,
        )?;

        unsafe { staging_buffer.write(device, pixels_rgba) }?;

        let image = Image::alloc(ImageCreateInfo {
            device,
            width,
            height,
            format,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            memory_property_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        })?;

        image.transition_layout(TransitionLayoutInfo {
            device,
            queue,
            transient_command_pool,
            format,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            sync: TransientCommandSync::default(),
        })?;

        unsafe {
            staging_buffer.copy_to_image(CopyToImageInfo {
                device,
                queue,
                transient_command_pool,
                image: image.image,
                width,
                height,
                sync: TransientCommandSync::default(),
            })
        }?;

        image.transition_layout(TransitionLayoutInfo {
            device,
            queue,
            transient_command_pool,
            format,
            old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            sync: TransientCommandSync::default(),
        })?;

        unsafe { staging_buffer.free(device) };

        // create image view
        let view = Image::alloc_view(device, image.image, format, vk::ImageAspectFlags::COLOR)?;

        // create sampler
        let sampler_create_info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SAMPLER_CREATE_INFO,
            p_next: ptr::null(),
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
