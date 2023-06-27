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
use vulkano::buffer::BufferContents;
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo
};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo};
use vulkano::sync::{self, GpuFuture};



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

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    let mut physical_devices = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices");

    ris_log::debug!("hello world");

    //    for (i, physical_device) in physical_devices.into_iter().enumerate() {
    //        ris_log::debug!("{} {:?}", i, physical_device.properties().device_name);
    //    }

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

    // let data: i32 = 12;
    // let buffer = Buffer::from_data(
    //     &memory_allocator,
    //     BufferCreateInfo {
    //         usage: BufferUsage::UNIFORM_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         usage: MemoryUsage::Upload,
    //         ..Default::default()
    //     },
    //     data,
    // )
    // .expect("failed to create buffer");

    // #[derive(BufferContents)]
    // #[repr(C)]
    // struct MyStruct {
    //     a: u32,
    //     b: u32,
    // }
    // 
    // let data = MyStruct { a: 13, b: 42 };
    // 
    // let buffer = Buffer::from_data(
    //     &memory_allocator,
    //     BufferCreateInfo {
    //         usage: BufferUsage::UNIFORM_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         usage: MemoryUsage::Upload,
    //         ..Default::default()
    //     },
    //     data,
    // )
    // .unwrap();

    // let iter = 0..128;
    // let buffer = Buffer::from_iter(
    //     &memory_allocator,
    //     BufferCreateInfo {
    //         usage: BufferUsage::UNIFORM_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         usage: MemoryUsage::Upload,
    //         ..Default::default()
    //     },
    //     iter,
    // )
    // .unwrap();

    // let mut content = buffer.write().unwrap();
    // content[12] = 83;
    // content[7] = 3;

    // ris_log::debug!("content {:?}", content);

    let source_content: Vec<i32> = (0..64).collect();
    let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();

    ris_log::debug!("source: {:?}", source_content);
    ris_log::debug!("target: {:?}", destination_content);

    let source = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        source_content,
    )
    .expect("failed to create source buffer");

    let destination = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Download,
            ..Default::default()
        },
        destination_content,
    )
    .expect("failed to create destination buffer");

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default()
    );

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone()))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let src_content = source.read().unwrap();
    let destination_content = destination.read().unwrap();

    ris_log::debug!("source: {:?}", &*src_content);
    ris_log::debug!("target: {:?}", &*destination_content);

    ris_log::debug!("we have reached the end");
    drop(log_guard);
    Ok(())
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

    let appenders: Appenders = vec![ConsoleAppender::new(), FileAppender::new(&app_info)];
    let log_guard = log::init(LogLevel::Trace, appenders);

    log_guard
}
