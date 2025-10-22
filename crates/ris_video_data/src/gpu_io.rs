use ash::vk;

use ris_error::prelude::*;

use super::buffer::Buffer;
use super::image::Image;
use super::transient_command::TransientCommand;
use super::transient_command::TransientCommandArgs;
use super::transient_command::TransientCommandSync;

pub struct GpuIOArgs<'a, GpuObject> {
    pub transient_command_args: TransientCommandArgs,
    pub bytes: Vec<u8>,
    pub gpu_object: &'a GpuObject,
    pub staging: &'a Buffer,
}

pub unsafe fn write_to_buffer(args: GpuIOArgs<Buffer>) -> RisResult<Vec<u8>> {
    let GpuIOArgs {
        transient_command_args,
        bytes: src,
        gpu_object: dst,
        staging,
    } = args;

    // setup
    ris_error::assert!(src.len() == dst.size())?;
    ris_error::assert!(src.len() <= staging.size())?;

    let device = transient_command_args.device.clone();

    // write to staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src.len() as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_from_nonoverlapping(src.as_ptr(), src.len());

    device.unmap_memory(staging.memory);

    // copy from staging buffer
    let args = transient_command_args.clone();
    let command = TransientCommand::begin(args)?;

    device.cmd_copy_buffer(
        command.buffer(),
        staging.buffer,
        dst.buffer,
        &[vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: src.len() as vk::DeviceSize,
        }],
    );

    // submit
    let sync = TransientCommandSync::default();
    let submit_future = command.end_and_submit(sync)?;
    submit_future.wait();

    // done
    Ok(src)
}

pub unsafe fn read_from_buffer(args: GpuIOArgs<Buffer>) -> RisResult<Vec<u8>> {
    let GpuIOArgs {
        transient_command_args,
        bytes: mut dst,
        gpu_object: src,
        staging,
    } = args;

    // setup
    ris_error::assert!(src.size() == dst.len())?;
    ris_error::assert!(src.size() <= staging.size())?;

    let device = transient_command_args.device.clone();

    // copy to staging buffer
    let args = transient_command_args.clone();
    let command = TransientCommand::begin(args)?;

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
    let sync = TransientCommandSync::default();
    let submit_future = command.end_and_submit(sync)?;
    submit_future.wait();

    // read from staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src.size() as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_to_nonoverlapping(dst.as_mut_ptr(), dst.len());

    device.unmap_memory(staging.memory);

    // done
    Ok(dst)
}

pub unsafe fn write_to_image(args: GpuIOArgs<Image>) -> RisResult<Vec<u8>> {
    let GpuIOArgs {
        transient_command_args,
        bytes: src,
        gpu_object: dst,
        staging,
    } = args;

    // setup
    ris_error::assert!(src.len() == dst.size())?;
    ris_error::assert!(src.len() <= staging.size())?;

    let device = transient_command_args.device.clone();

    // write to staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src.len() as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_from_nonoverlapping(src.as_ptr(), src.len());

    device.unmap_memory(staging.memory);

    // copy from staging buffer
    let args = transient_command_args.clone();
    let command = TransientCommand::begin(args)?;

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
    let sync = TransientCommandSync::default();
    let submit_future = command.end_and_submit(sync)?;
    submit_future.wait();

    // done
    Ok(src)
}

pub unsafe fn read_from_image(args: GpuIOArgs<Image>) -> RisResult<Vec<u8>> {
    let GpuIOArgs {
        transient_command_args,
        bytes: mut dst,
        gpu_object: src,
        staging,
    } = args;

    // setup
    ris_error::assert!(src.size() == dst.len())?;
    ris_error::assert!(src.size() <= staging.size())?;

    let device = transient_command_args.device.clone();

    // copy to staging buffer
    let args = transient_command_args.clone();
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
    let sync = TransientCommandSync::default();
    let submit_future = command.end_and_submit(sync)?;
    submit_future.wait();

    // read from staging buffer
    let ptr = device.map_memory(
        staging.memory,
        0,
        src.size() as vk::DeviceSize,
        vk::MemoryMapFlags::empty(),
    )? as *mut u8;

    ptr.copy_to_nonoverlapping(dst.as_mut_ptr(), dst.len());

    device.unmap_memory(staging.memory);

    // done
    Ok(dst)
}
