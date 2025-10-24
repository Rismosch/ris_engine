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
    let size = std::mem::size_of_val(src) as vk::DeviceSize;

    let ptr = device.map_memory(
        memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )? as *mut T;
    ptr.copy_from_nonoverlapping(src.as_ptr(), src.len());
    device.unmap_memory(memory);
    Ok(())
}

pub unsafe fn write_to_buffer<T>(args: GpuIOArgs<Buffer, impl AsRef<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values,
        gpu_object,
        staging,
    } = args;

    // setup
    let (src_ptr, src_len) = fat_ptr_ref(values);
    let dst = gpu_object;

    ris_error::assert!(src_len == dst.size())?;
    ris_error::assert!(src_len <= staging.size())?;

    let device = transient_command_args.device.clone();
    let tcas = transient_command_args.clone();
    let sync = TransientCommandSync::default();

    // write to staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src_len as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_from_nonoverlapping(src_ptr, src_len);

    device.unmap_memory(staging.memory);

    // copy from staging buffer
    let command = TransientCommand::begin(tcas)?;

    device.cmd_copy_buffer(
        command.buffer(),
        staging.buffer,
        dst.buffer,
        &[vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: src_len as vk::DeviceSize,
        }],
    );

    // submit
    let future = command.end_and_submit(sync)?;
    future.wait();

    Ok(())
}

pub unsafe fn read_from_buffer<T>(args: GpuIOArgs<Buffer, impl AsMut<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values,
        gpu_object,
        staging,
    } = args;

    // setup
    let src = gpu_object;
    let (dst_ptr, dst_len) = fat_ptr_mut(values);

    ris_error::assert!(src.size() == dst_len)?;
    ris_error::assert!(src.size() <= staging.size())?;

    let device = transient_command_args.device.clone();
    let tcas = transient_command_args.clone();
    let sync = TransientCommandSync::default();

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
    let submit_future = command.end_and_submit(sync)?;
    submit_future.wait();

    // read from staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src.size() as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_to_nonoverlapping(dst_ptr, dst_len);

    device.unmap_memory(staging.memory);

    Ok(())
}

pub unsafe fn write_to_image<T>(args: GpuIOArgs<Image, impl AsRef<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values,
        gpu_object,
        staging,
    } = args;

    // setup
    let (src_ptr, src_len) = fat_ptr_ref(values);
    let dst = gpu_object;

    ris_error::assert!(src_len == dst.size())?;
    ris_error::assert!(src_len <= staging.size())?;

    let device = transient_command_args.device.clone();
    let tcas = transient_command_args.clone();
    let sync = TransientCommandSync::default();

    // write to staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src_len as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_from_nonoverlapping(src_ptr, src_len);

    device.unmap_memory(staging.memory);

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
    let submit_future = command.end_and_submit(sync)?;
    submit_future.wait();

    Ok(())
}

pub unsafe fn read_from_image<T>(args: GpuIOArgs<Image, impl AsMut<[T]>>) -> RisResult<()> {
    let GpuIOArgs {
        transient_command_args,
        values,
        gpu_object,
        staging,
    } = args;

    // setup
    let src = gpu_object;
    let (dst_ptr, dst_len) = fat_ptr_mut(values);

    ris_error::assert!(src.size() == dst_len)?;
    ris_error::assert!(src.size() <= staging.size())?;

    let device = transient_command_args.device.clone();
    let args = transient_command_args.clone();
    let sync = TransientCommandSync::default();

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
    let submit_future = command.end_and_submit(sync)?;
    submit_future.wait();

    // read from staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src.size() as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_to_nonoverlapping(dst_ptr, dst_len);

    device.unmap_memory(staging.memory);

    Ok(())
}

fn fat_ptr_ref<T>(value: impl AsRef<[T]>) -> (*const u8, usize) {
    let slice = value.as_ref();
    let ptr = slice.as_ptr() as *const u8;
    let len = slice.len() * std::mem::size_of::<T>();
    (ptr, len)
}

fn fat_ptr_mut<T>(mut value: impl AsMut<[T]>) -> (*mut u8, usize) {
    let slice = value.as_mut();
    let ptr = slice.as_mut_ptr() as *mut u8;
    let len = slice.len() * std::mem::size_of::<T>();
    (ptr, len)
}
