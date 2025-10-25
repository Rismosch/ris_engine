use ash::vk;

use ris_error::prelude::*;

use super::buffer::Buffer;
use super::image::Image;
use super::transient_command::TransientCommand;
use super::transient_command::TransientCommandArgs;
use super::transient_command::TransientCommandSync;

pub struct GpuIOArgs<'a, GpuObject, Bytes> {
    pub transient_command_args: TransientCommandArgs,
    pub values: Bytes,
    pub gpu_object: &'a GpuObject,
    pub staging: &'a Buffer,
}

pub unsafe fn write_to_memory<T>(
    device: &ash::Device,
    values: impl AsRef<[T]>,
    memory: vk::DeviceMemory,
) -> RisResult<()> {
    let src = values.as_ref();

    let mapped_memory = device.map_memory(
        memory,
        0,
        vk::WHOLE_SIZE,
        vk::MemoryMapFlags::empty(),
    )? as *mut T;

    write_to_mapped_memory(
        device,
        src,
        memory,
        mapped_memory,
    )?;

    device.unmap_memory(memory);
    Ok(())
}

pub unsafe fn write_to_mapped_memory<T>(
    device: &ash::Device,
    values: impl AsRef<[T]>,
    memory: vk::DeviceMemory,
    mapped_memory: *mut T,
) -> RisResult<()> {
    let src = values.as_ref();

    mapped_memory.copy_from_nonoverlapping(src.as_ptr(), src.len());

    device.flush_mapped_memory_ranges(&[vk::MappedMemoryRange{
        s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
        p_next: std::ptr::null(),
        memory: memory,
        offset: 0,
        size: vk::WHOLE_SIZE,
    }])?;

    Ok(())
}

pub unsafe fn read_from_memory<T>(
    device: &ash::Device,
    mut values: impl AsMut<[T]>,
    memory: vk::DeviceMemory,
) -> RisResult<()> {
    let dst = values.as_mut();

    let mapped_memory = device.map_memory(
        memory,
        0,
        vk::WHOLE_SIZE,
        vk::MemoryMapFlags::empty(),
    )? as *mut T;

    read_from_mapped_memory(
        device,
        dst,
        memory,
        mapped_memory,
    )?;

    device.unmap_memory(memory);
    Ok(())
}

pub unsafe fn read_from_mapped_memory<T>(
    device: &ash::Device,
    mut values: impl AsMut<[T]>,
    memory: vk::DeviceMemory,
    mapped_memory: *mut T,
) -> RisResult<()> {
    let dst = values.as_mut();

    device.invalidate_mapped_memory_ranges(&[vk::MappedMemoryRange{
        s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
        p_next: std::ptr::null(),
        memory,
        offset: 0,
        size: vk::WHOLE_SIZE,
    }])?;

    mapped_memory.copy_to_nonoverlapping(dst.as_mut_ptr(), dst.len());

    Ok(())
}

pub unsafe fn write_to_buffer<T>(args: GpuIOArgs<Buffer, impl AsRef<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values: src,
        gpu_object: dst,
        staging,
    } = args;

    // setup
    let src = src.as_ref();
    let src_size = std::mem::size_of_val(src);

    ris_error::assert!(src_size == dst.size())?;
    ris_error::assert!(src_size <= staging.size())?;

    let device = transient_command_args.device.clone();
    let tcas = transient_command_args.clone();

    // write to staging buffer
    write_to_memory(&device, src, staging.memory)?;

    // copy from staging buffer
    let command = TransientCommand::begin(tcas)?;

    device.cmd_copy_buffer(
        command.buffer(),
        staging.buffer,
        dst.buffer,
        &[vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: src_size as vk::DeviceSize,
        }],
    );

    // submit
    submit(&device, command)?;

    Ok(())
}

pub unsafe fn read_from_buffer<T>(args: GpuIOArgs<Buffer, impl AsMut<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values: mut dst,
        gpu_object: src,
        staging,
    } = args;

    // setup
    let dst = dst.as_mut();
    let dst_size = std::mem::size_of_val(dst);

    ris_error::assert!(src.size() == dst_size)?;
    ris_error::assert!(src.size() <= staging.size())?;

    let device = transient_command_args.device.clone();
    let tcas = transient_command_args.clone();

    // copy to staging buffer
    let command = TransientCommand::begin(tcas)?;

    device.cmd_copy_buffer(
        command.buffer(),
        src.buffer,
        staging.buffer,
        &[vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: src.size() as vk::DeviceSize,
        }],
    );

    // submit
    submit(&device, command)?;

    // read from staging buffer
    read_from_memory(&device, dst, staging.memory)?;

    Ok(())
}

pub unsafe fn write_to_image<T>(args: GpuIOArgs<Image, impl AsRef<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values: src,
        gpu_object: dst,
        staging,
    } = args;

    // setup
    let src = src.as_ref();
    let src_size = std::mem::size_of_val(src);

    ris_error::assert!(src_size == dst.size())?;
    ris_error::assert!(src_size <= staging.size())?;

    let device = transient_command_args.device.clone();
    let tcas = transient_command_args.clone();

    // write to staging buffer
    write_to_memory(&device, src, staging.memory)?;

    // copy from staging buffer
    let command = TransientCommand::begin(tcas)?;

    device.cmd_copy_buffer_to_image(
        command.buffer(),
        staging.buffer,
        dst.image,
        dst.layout(),
        &[vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
            image_extent: vk::Extent3D {
                width: dst.width() as u32,
                height: dst.height() as u32,
                depth: 1,
            },
        }],
    );

    // submit
    submit(&device, command)?;

    Ok(())
}

pub unsafe fn read_from_image<T>(args: GpuIOArgs<Image, impl AsMut<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values: mut dst,
        gpu_object: src,
        staging,
    } = args;

    // setup
    let dst = dst.as_mut();
    let dst_size = std::mem::size_of_val(dst);

    ris_error::assert!(src.size() == dst_size)?;
    ris_error::assert!(src.size() <= staging.size())?;

    let device = transient_command_args.device.clone();
    let args = transient_command_args.clone();

    // copy to staging buffer
    let command = TransientCommand::begin(args)?;

    device.cmd_copy_image_to_buffer(
        command.buffer(),
        src.image,
        src.layout(),
        staging.buffer,
        &[vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
            image_extent: vk::Extent3D {
                width: src.width() as u32,
                height: src.height() as u32,
                depth: 1,
            },
        }],
    );

    // submit
    submit(&device, command)?;

    // read from staging buffer
    read_from_memory(&device, dst, staging.memory)?;

    Ok(())
}

unsafe fn submit(device: &ash::Device, command: TransientCommand) -> RisResult<()>{
    let fence_create_info = vk::FenceCreateInfo {
        s_type: vk::StructureType::FENCE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::FenceCreateFlags::empty(),
    };
    let fence = device.create_fence(&fence_create_info, None)?;

    let sync = TransientCommandSync{
        wait: Vec::with_capacity(0),
        dst: Vec::with_capacity(0),
        signal: Vec::with_capacity(0),
        fence,
    };

    command.end_and_submit(sync)?;

    device.wait_for_fences(&[fence], true, u64::MAX)?;
    device.destroy_fence(fence, None);

    Ok(())
}
