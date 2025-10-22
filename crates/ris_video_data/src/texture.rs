use ash::vk;

use ris_error::RisResult;

use super::buffer::Buffer;
use super::buffer::CopyToImageInfo;
use super::image::Image;
use super::image::ImageCreateInfo;
use super::image::TransitionLayoutInfo;
use super::transient_command::prelude::*;

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
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&self, device: &ash::Device) {
        device.destroy_sampler(self.sampler, None);
        device.destroy_image_view(self.view, None);

        self.image.free(device);
    }

    pub fn alloc(info: TextureCreateInfo) -> RisResult<Self> {
        unsafe {
            use super::gpu_io;
            use super::gpu_io::GpuIOArgs;
            use super::transient_command::prelude::*;

            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

            let staging = Buffer::alloc_staging(
                info.device,
                data.len(),
                info.physical_device_memory_properties,
            )?;

            let buffer = Buffer::alloc(
                info.device,
                data.len(),
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
                info.physical_device_memory_properties,
            )?;

            let tcas = TransientCommandArgs {
                device: info.device.clone(),
                queue: info.queue,
                command_pool: info.transient_command_pool,
            };

            let r1 = gpu_io::write_to_buffer(GpuIOArgs {
                transient_command_args: tcas.clone(),
                bytes: data.clone(),
                gpu_object: &buffer,
                staging: &staging,
            })?;

            let r2 = gpu_io::read_from_buffer(GpuIOArgs {
                transient_command_args: tcas.clone(),
                bytes: vec![0; data.len()],
                gpu_object: &buffer,
                staging: &staging,
            })?;

            let mut image = Image::alloc(ImageCreateInfo {
                device: info.device,
                width: 5,
                height: 2,
                format: vk::Format::R8_UINT,
                usage: vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST,
                physical_device_memory_properties: info.physical_device_memory_properties,
            })?;

            image.transition_layout(TransitionLayoutInfo {
                transient_command_args: tcas.clone(),
                new_layout: vk::ImageLayout::GENERAL,
                sync: TransientCommandSync::default(),
            })?;

            let r3 = gpu_io::write_to_image(GpuIOArgs {
                transient_command_args: tcas.clone(),
                bytes: data.clone(),
                gpu_object: &image,
                staging: &staging,
            })?;

            let r4 = gpu_io::read_from_image(GpuIOArgs {
                transient_command_args: tcas.clone(),
                bytes: vec![0; data.len()],
                gpu_object: &image,
                staging: &staging,
            })?;

            println!("data: {:?}", data);
            println!("r1:   {:?}", r1);
            println!("r2:   {:?}", r2);
            println!("r3:   {:?}", r3);
            println!("r4:   {:?}", r4);
        }

        panic!("memory io tests passed");

        todo!();
        //let TextureCreateInfo {
        //    device,
        //    queue,
        //    transient_command_pool,
        //    physical_device_memory_properties,
        //    physical_device_properties,
        //    width,
        //    height,
        //    format,
        //    filter,
        //    pixels_rgba,
        //} = info;

        //ris_error::debug_assert!(width != 0)?;
        //ris_error::debug_assert!(height != 0)?;

        //let actual_len = pixels_rgba.len();
        //let expected_len = (width * height * 4) as usize;
        //ris_error::debug_assert!(actual_len == expected_len)?;

        //// create image and copy asset to it
        //let staging_buffer = Buffer::alloc(
        //    device,
        //    pixels_rgba.len() as vk::DeviceSize,
        //    vk::BufferUsageFlags::TRANSFER_SRC,
        //    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        //    physical_device_memory_properties,
        //)?;

        //unsafe { staging_buffer.write(device, pixels_rgba) }?;

        //let image = Image::alloc(ImageCreateInfo {
        //    device,
        //    width,
        //    height,
        //    format,
        //    tiling: vk::ImageTiling::OPTIMAL,
        //    usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
        //    memory_property_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
        //    physical_device_memory_properties,
        //})?;

        //image.transition_layout(TransitionLayoutInfo {
        //    device,
        //    queue,
        //    transient_command_pool,
        //    format,
        //    old_layout: vk::ImageLayout::UNDEFINED,
        //    new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        //    sync: TransientCommandSync::default(),
        //})?;

        //unsafe {
        //    staging_buffer.copy_to_image(CopyToImageInfo {
        //        device,
        //        queue,
        //        transient_command_pool,
        //        image: image.image,
        //        width,
        //        height,
        //        sync: TransientCommandSync::default(),
        //    })
        //}?;

        //image.transition_layout(TransitionLayoutInfo {
        //    device,
        //    queue,
        //    transient_command_pool,
        //    format,
        //    old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        //    new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        //    sync: TransientCommandSync::default(),
        //})?;

        //unsafe { staging_buffer.free(device) };

        //// create image view
        //let view = Image::alloc_view(device, image.image, format, vk::ImageAspectFlags::COLOR)?;

        //// create sampler
        //let sampler_create_info = vk::SamplerCreateInfo {
        //    s_type: vk::StructureType::SAMPLER_CREATE_INFO,
        //    p_next: ptr::null(),
        //    flags: vk::SamplerCreateFlags::empty(),
        //    mag_filter: filter,
        //    min_filter: filter,
        //    mipmap_mode: vk::SamplerMipmapMode::NEAREST,
        //    address_mode_u: vk::SamplerAddressMode::REPEAT,
        //    address_mode_v: vk::SamplerAddressMode::REPEAT,
        //    address_mode_w: vk::SamplerAddressMode::REPEAT,
        //    mip_lod_bias: 0.0,
        //    anisotropy_enable: vk::TRUE,
        //    max_anisotropy: physical_device_properties.limits.max_sampler_anisotropy,
        //    compare_enable: vk::FALSE,
        //    compare_op: vk::CompareOp::ALWAYS,
        //    min_lod: 0.0,
        //    max_lod: 0.0,
        //    border_color: vk::BorderColor::INT_OPAQUE_BLACK,
        //    unnormalized_coordinates: vk::FALSE,
        //};

        //let sampler = unsafe { device.create_sampler(&sampler_create_info, None) }?;

        //Ok(Self {
        //    image,
        //    view,
        //    sampler,
        //})
    }
}
