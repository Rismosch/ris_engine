use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use ash::vk;

use ris_asset::codecs::qoi;
use ris_asset::AssetId;
use ris_error::Extensions;
use ris_error::RisResult;

use super::buffer::Buffer;
use super::image::Image;

pub struct Texture {
    pub image: Image,
    pub view: vk::ImageView,
    pub sampler: vk::Sampler,
}

impl Texture {
    pub fn alloc(
        device: &ash::Device,
        queue: vk::Queue,
        transient_command_pool: vk::CommandPool,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        physical_device_properties: vk::PhysicalDeviceProperties,
        width: u32,
        height: u32,
        pixels_rgba: &[u8],
    ) -> RisResult<Self> { 
        let actual_len = pixels_rgba.len();
        let expected_len = (width * height * 4) as usize;
        ris_error::debug_assert!(actual_len == expected_len)?;

        // create image and copy asset to it
        let staging_buffer = Buffer::alloc(
            &device,
            pixels_rgba.len() as vk::DeviceSize,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            physical_device_memory_properties,
        )?;

        staging_buffer.write(&device, &pixels_rgba)?;

        let image = Image::alloc(
            &device,
            width,
            height,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        image.transition_layout(
            &device,
            queue,
            transient_command_pool,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        )?;

        staging_buffer.copy_to_image(
            &device,
            queue,
            transient_command_pool,
            image.image,
            width,
            height,
        )?;

        image.transition_layout(
            &device,
            queue,
            transient_command_pool,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;

        staging_buffer.free(&device);

        // create image view
        let view = Image::alloc_view(
            device,
            image.image,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageAspectFlags::COLOR,
        )?;

        // create sampler
        let sampler_create_info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SAMPLER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SamplerCreateFlags::empty(),
            mag_filter: vk::Filter::LINEAR,
            min_filter: vk::Filter::LINEAR,
            mipmap_mode: vk::SamplerMipmapMode::LINEAR,
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

    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_sampler(self.sampler, None);
            device.destroy_image_view(self.view, None);
        }

        self.image.free(device);
    }
}
