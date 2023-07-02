use ris_core::engine::Engine;
use ris_data::info::app_info::AppInfo;
use ris_data::info::args_info::ArgsInfo;
use ris_data::info::build_info::BuildInfo;
use ris_data::info::cpu_info::CpuInfo;
use ris_data::info::file_info::FileInfo;
use ris_data::info::package_info::PackageInfo;
use ris_data::info::sdl_info::SdlInfo;
use ris_data::package_info;
use ris_log::{
    log::{self, Appenders, LogGuard},
    log_level::LogLevel,
};
use ris_util::{throw, unwrap_or_throw};
use vulkano::device::QueueFlags;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::VulkanLibrary;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryUsage};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo
};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, CopyBufferInfo, CopyImageToBufferInfo};
use vulkano::sync::{self, GpuFuture};
use vulkano::pipeline::ComputePipeline;
use vulkano::pipeline::Pipeline;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::pipeline::PipelineBindPoint;
use vulkano::image::{ImageDimensions, StorageImage};
use vulkano::format::{Format, ClearColorValue};
use image::{ImageBuffer, Rgba};
use vulkano::image::view::ImageView;


pub const RESTART_CODE: i32 = 42;

fn main() -> Result<(), String> {
    let app_info = get_app_info()?;
    //
    //    if app_info.args.no_restart {
    //        run(app_info)
    //    } else {
    //        wrap(app_info)
    //    }

    let log_guard = init_log(&app_info);

    ris_log::debug!("hello world");
    
    //computepipeline();
    // images();
    fractal();

    ris_log::debug!("we have reached the end");
    drop(log_guard);
    Ok(())
}

fn computepipeline(){

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    let mut physical_devices = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices");

    let physical_device = physical_devices.next().expect("no devices available");

    for family in physical_device.queue_family_properties() {
        ris_log::debug!(
            "found a queue family with {:?} queue(s)",
            family.queue_count
        );
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    ris_log::debug!("queue family index: {}", queue_family_index);

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    let data_iter = 0..65536u32;
    let data_buffer = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        data_iter,
    )
    .expect("failed to create buffer");

    let source = "
        #version 460

        layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

        layout(set = 0, binding = 0) buffer Data {
            uint data[];
        } buf;

        void main() {
            uint idx = gl_GlobalInvocationID.x;
            buf.data[idx] *= 12;
        }
    ";

    let mut compiler = shaderc::Compiler::new().expect("could not create compiler");
    let mut options = shaderc::CompileOptions::new().expect("coult not create options");
    options.add_macro_definition("EP", Some("main"));
    let binary_result = compiler.compile_into_spirv(
        source, shaderc::ShaderKind::Compute,
        "shader.glsl", "main", Some(&options)).expect("could not compile into spirv");

    ris_log::debug!("binary: {:?}", binary_result.as_binary());

    let words: &[u32] = binary_result.as_binary();
    let shader_module = unsafe {vulkano::shader::ShaderModule::from_words(
        device.clone(),
        words
    )}.expect("could not load shader module");
    let entry_point = shader_module.entry_point("main").unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        entry_point,
        &(),
        None,
        |_| {},
    )
    .expect("failed to create compute pipeline");

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
    let pipeline_layout = compute_pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();

    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts
        .get(descriptor_set_layout_index)
        .unwrap();
    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::buffer(0, data_buffer.clone())],
    )
    .unwrap();

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let work_group_counts = [1024, 1, 1];

    command_buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            descriptor_set_layout_index as u32,
            descriptor_set,
        )
        .dispatch(work_group_counts)
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    ris_log::debug!("result: {:?}", &*content);
}

fn images(){
    ris_log::debug!("images");

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    let mut physical_devices = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices");

    let physical_device = physical_devices.next().expect("no devices available");

    for family in physical_device.queue_family_properties() {
        ris_log::debug!(
            "found a queue family with {:?} queue(s)",
            family.queue_count
        );
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    ris_log::debug!("queue family index: {}", queue_family_index);

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    let image = StorageImage::new(
        &memory_allocator,
        ImageDimensions::Dim2d {
            width: 1024,
            height: 1024,
            array_layers: 1,
        },
        Format::R8G8B8A8_UNORM,
        Some(queue.queue_family_index()),
    )
    .unwrap();

    let buf = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Download,
            ..Default::default()
        },
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .clear_color_image(ClearColorImageInfo {
            clear_value: ClearColorValue::Float([1.0, 0.0, 1.0, 1.0]),
            ..ClearColorImageInfo::image(image.clone())
        })
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            buf.clone(),
        ))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    ris_log::debug!("buffer content: {:?}", &*buffer_content);
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();
}

fn fractal() {
    ris_log::debug!("images");

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    let mut physical_devices = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices");

    let physical_device = physical_devices.next().expect("no devices available");

    for family in physical_device.queue_family_properties() {
        ris_log::debug!(
            "found a queue family with {:?} queue(s)",
            family.queue_count
        );
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    ris_log::debug!("queue family index: {}", queue_family_index);

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    let source = "
        #version 460
        
        layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
        
        layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;
        
        void main() {
            vec2 norm_coordinates = (gl_GlobalInvocationID.xy + vec2(0.5)) / vec2(imageSize(img));
            vec2 c = (norm_coordinates - vec2(0.5)) * 2.0 - vec2(1.0, 0.0);
        
            vec2 z = vec2(0.0, 0.0);
            float i;
            for (i = 0.0; i < 1.0; i += 0.005) {
                z = vec2(
                    z.x * z.x - z.y * z.y + c.x,
                    z.y * z.x + z.x * z.y + c.y
                );
        
                if (length(z) > 4.0) {
                    break;
                }
            }
        
            vec4 to_write = vec4(vec3(i), 1.0);
            imageStore(img, ivec2(gl_GlobalInvocationID.xy), to_write);
        }
    ";

    let mut compiler = shaderc::Compiler::new().expect("could not create compiler");
    let mut options = shaderc::CompileOptions::new().expect("coult not create options");
    options.add_macro_definition("EP", Some("main"));
    let binary_result = compiler.compile_into_spirv(
        source, shaderc::ShaderKind::Compute,
        "shader.glsl", "main", Some(&options)).expect("could not compile into spirv");

    ris_log::debug!("binary: {:?}", binary_result.as_binary());

    let words: &[u32] = binary_result.as_binary();
    let shader_module = unsafe {vulkano::shader::ShaderModule::from_words(
        device.clone(),
        words
    )}.expect("could not load shader module");
    let entry_point = shader_module.entry_point("main").unwrap();

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    let image = StorageImage::new(
        &memory_allocator,
        ImageDimensions::Dim2d {
            width: 1024,
            height: 1024,
            array_layers: 1,
        },
        Format::R8G8B8A8_UNORM,
        Some(queue.queue_family_index()),
    )
    .unwrap();

    let view = ImageView::new_default(image.clone()).unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        entry_point,
        &(),
        None,
        |_| {},
    )
    .expect("failed to create compute pipeline");

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [WriteDescriptorSet::image_view(0, view.clone())],
    )
    .unwrap();

    let buf = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Download,
            ..Default::default()
        },
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );
    let mut builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0,
            set,
        )
        .dispatch([1024 / 8, 1024 / 8, 1])
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                image.clone(),
                buf.clone(),
        ))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();

    ris_log::debug!("success");
}

fn get_app_info() -> Result<AppInfo, String> {
    let cpu_info = CpuInfo::new();

    let args_info_result = ArgsInfo::new(&cpu_info);
    let args_info = match args_info_result {
        Ok(args) => args,
        Err(error) => return Err(format!("error while parsing cli args: {}", error)),
    };

    let package_info = package_info!();
    let build_info = BuildInfo::new();
    let file_info = FileInfo::new(&package_info);
    let sdl_info = SdlInfo::new();

    Ok(AppInfo::new(
        args_info,
        package_info,
        build_info,
        file_info,
        sdl_info,
        cpu_info,
    ))
}

fn run(app_info: AppInfo) -> Result<(), String> {
    let log_guard = init_log(&app_info);

    let mut engine = Engine::new(app_info)?;
    let result = engine.run();

    if let Err(error) = result {
        ris_log::fatal!("error while running engine: \"{}\"", error);
    }

    drop(log_guard);

    if engine.wants_to_restart {
        std::process::exit(RESTART_CODE);
    }

    Ok(())
}

fn wrap(mut app_info: AppInfo) -> Result<(), String> {
    app_info.args.no_restart = true;

    let executable_path = &app_info.args.executable_path;
    let raw_args = app_info.args.generate_raw_args();

    loop {
        let mut command = std::process::Command::new(executable_path);

        for arg in raw_args.iter().skip(1) {
            command.arg(arg);
        }

        let child = unwrap_or_throw!(command.spawn(), "child could not be spawned");
        let output = unwrap_or_throw!(child.wait_with_output(), "child could not be awaited");

        let exit_code = if let Some(code) = output.status.code() {
            println!("process finished with code {}", code);

            if code == RESTART_CODE {
                println!("restarting...");
                continue;
            } else {
                Some(code)
            }
        } else {
            println!("process finished with no code");
            None
        };

        if output.status.success() {
            return Ok(());
        } else {
            let output_bytes = output.stderr;
            let output_string = String::from_utf8(output_bytes);

            match output_string {
                Ok(to_print) => eprintln!("{}", to_print),
                Err(error) => throw!("error while formatting output.stderr: {}", error),
            }

            match exit_code {
                Some(code) => std::process::exit(code),
                None => return Err(String::from("no code to exit from")),
            }
        }
    }
}

fn init_log(app_info: &AppInfo) -> LogGuard {
    use ris_core::appenders::{console_appender::ConsoleAppender, file_appender::FileAppender};

    let appenders: Appenders = vec![ConsoleAppender::new(), FileAppender::new(app_info)];
    log::init(LogLevel::Trace, appenders)
}
