use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, logic_data::LogicData,
    output_data::OutputData,
};
//use vulkano::buffer::BufferContents;
//use vulkano::pipeline::graphics::vertex_input::Vertex;
//use vulkano::pipeline::Pipeline;
//use vulkano::sync::GpuFuture;

pub struct OutputFrame {}

//#[derive(BufferContents, Vertex)]
//#[repr(C)]
//struct MyVertex {
//    #[format(R32G32_SFLOAT)]
//    position: [f32; 2],
//}

impl OutputFrame {
    pub fn new() -> Result<Self, String> {
        //// init vulkan
        //let library = vulkano::VulkanLibrary::new().map_err(|_| "no local Vulkan library/DLL")?;
        //let instance = vulkano::instance::Instance::new(
        //    library,
        //    vulkano::instance::InstanceCreateInfo::default(),
        //)
        //.map_err(|_| "failed to create instance")?;

        //// create device
        //let mut physical_devices = instance
        //    .enumerate_physical_devices()
        //    .map_err(|_| "could not enumerate devices")?;
        //let physical_device = physical_devices.next().ok_or("no devices available")?;

        //let queue_family_index = physical_device
        //    .queue_family_properties()
        //    .iter()
        //    .enumerate()
        //    .position(|(_queue_family_index, queue_family_properties)| {
        //        queue_family_properties
        //            .queue_flags
        //            .contains(vulkano::device::QueueFlags::GRAPHICS)
        //    })
        //    .ok_or("could not find a graphical queue family")?
        //    as u32;

        //let (device, mut queues) = vulkano::device::Device::new(
        //    physical_device,
        //    vulkano::device::DeviceCreateInfo {
        //        queue_create_infos: vec![vulkano::device::QueueCreateInfo {
        //            queue_family_index,
        //            ..Default::default()
        //        }],
        //        ..Default::default()
        //    },
        //)
        //.map_err(|_| "failed to create device")?;

        //let queue = queues.next().ok_or("no queues available")?;

        //// allocators
        //let memory_allocator =
        //    vulkano::memory::allocator::StandardMemoryAllocator::new_default(device.clone());
        //let descriptor_set_allocator =
        //    vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator::new(device.clone());
        //let command_buffer_allocator = vulkano::command_buffer::allocator::StandardCommandBufferAllocator::new(
        //    device.clone(),
        //    vulkano::command_buffer::allocator::StandardCommandBufferAllocatorCreateInfo::default()
        //);

        //// triangle
        //let vertex1 = MyVertex {
        //    position: [0.0, -0.5],
        //};
        //let vertex2 = MyVertex {
        //    position: [-0.5, 0.5],
        //};
        //let vertex3 = MyVertex {
        //    position: [0.5, 0.5],
        //};

        //let vertex_buffer = vulkano::buffer::Buffer::from_iter(
        //    &memory_allocator,
        //    vulkano::buffer::BufferCreateInfo {
        //        usage: vulkano::buffer::BufferUsage::VERTEX_BUFFER,
        //        ..Default::default()
        //    },
        //    vulkano::memory::allocator::AllocationCreateInfo {
        //        usage: vulkano::memory::allocator::MemoryUsage::Upload,
        //        ..Default::default()
        //    },
        //    vec![vertex1, vertex2, vertex3],
        //)
        //.map_err(|_| "could not create vertex buffer")?;

        //// shaders
        //let vertex_source = "
        //    #version 460

        //    layout(location = 0) in vec2 position;

        //    void main() {
        //        gl_Position = vec4(position, 0.0, 1.0);
        //    }
        //";

        //let fragment_source = "
        //    #version 460

        //    layout(location = 0) out vec4 f_color;

        //    void main() {
        //        f_color = vec4(1.0, 0.0, 0.0, 1.0);
        //    }
        //";

        //let compiler = shaderc::Compiler::new().ok_or("could not initialize shaderc compiler")?;
        //let options =
        //    shaderc::CompileOptions::new().ok_or("could not initialize shaderc options")?;

        //let vertex_artifact = compiler
        //    .compile_into_spirv(
        //        vertex_source,
        //        shaderc::ShaderKind::Vertex,
        //        "vertex.glsl",
        //        "main",
        //        Some(&options),
        //    )
        //    .map_err(|_| "could not compile vertex shader")?;
        //let vertex_words: &[u32] = vertex_artifact.as_binary();
        //let vertex_module =
        //    unsafe { vulkano::shader::ShaderModule::from_words(device.clone(), vertex_words) }
        //        .map_err(|_| "could not load vertex shader module")?;
        //let vertex_entry_point = vertex_module.entry_point("main").unwrap();

        //let fragment_artifact = compiler
        //    .compile_into_spirv(
        //        fragment_source,
        //        shaderc::ShaderKind::Fragment,
        //        "fragment.glsl",
        //        "main",
        //        Some(&options),
        //    )
        //    .map_err(|_| "could not compile fragment shader")?;
        //let fragment_words: &[u32] = fragment_artifact.as_binary();
        //let fragment_module =
        //    unsafe { vulkano::shader::ShaderModule::from_words(device.clone(), fragment_words) }
        //        .map_err(|_| "could not load fragment shader module")?;
        //let fragment_entry_point = fragment_module.entry_point("main").unwrap();

        //// render pass
        //let render_pass = vulkano::single_pass_renderpass!(
        //    device.clone(),
        //    attachments: {
        //        color: {
        //            load: Clear,
        //            store: Store,
        //            format: vulkano::format::Format::R8G8B8A8_UNORM,
        //            samples: 1,
        //        },
        //    },
        //    pass: {
        //        color: [color],
        //        depth_stencil: {},
        //    },
        //)
        //.map_err(|_| "could not create render pass")?;

        //let image = vulkano::image::StorageImage::new(
        //    &memory_allocator,
        //    vulkano::image::ImageDimensions::Dim2d {
        //        width: 1024,
        //        height: 1024,
        //        array_layers: 1,
        //    },
        //    vulkano::format::Format::R8G8B8A8_UNORM,
        //    Some(queue.queue_family_index()),
        //)
        //.map_err(|_| "could not create image")?;
        //let view = vulkano::image::view::ImageView::new_default(image.clone())
        //    .map_err(|_| "could not create image view")?;
        //let framebuffer = vulkano::render_pass::Framebuffer::new(
        //    render_pass.clone(),
        //    vulkano::render_pass::FramebufferCreateInfo {
        //        attachments: vec![view],
        //        ..Default::default()
        //    },
        //)
        //.map_err(|_| "could not create frame buffer")?;

        //// pipeline
        //let viewport = vulkano::pipeline::graphics::viewport::Viewport {
        //    origin: [0.0, 0.0],
        //    dimensions: [1024.0, 1024.0],
        //    depth_range: 0.0..1.0,
        //};

        //let pipeline = vulkano::pipeline::GraphicsPipeline::start()
        //    .vertex_input_state(MyVertex::per_vertex())
        //    .vertex_shader(vertex_entry_point, ())
        //    .input_assembly_state(vulkano::pipeline::graphics::input_assembly::InputAssemblyState::new())
        //    .viewport_state(vulkano::pipeline::graphics::viewport::ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
        //    .fragment_shader(fragment_entry_point,())
        //    .render_pass(vulkano::render_pass::Subpass::from(render_pass.clone(), 0).ok_or("could not create pipeline sub pass")?)
        //    .build(device.clone())
        //    .map_err(|_| "could not build graphics pipeline")?;

        //// this is only needed to read out rendered image
        //// remove once image is forwarded to SDL2
        //let buf = vulkano::buffer::Buffer::from_iter(
        //    &memory_allocator,
        //    vulkano::buffer::BufferCreateInfo {
        //        usage: vulkano::buffer::BufferUsage::TRANSFER_DST,
        //        ..Default::default()
        //    },
        //    vulkano::memory::allocator::AllocationCreateInfo {
        //        usage: vulkano::memory::allocator::MemoryUsage::Download,
        //        ..Default::default()
        //    },
        //    (0..1024 * 1024 * 4).map(|_| 0u8),
        //)
        //.map_err(|_| "could not create buffer to download image")?;

        //// dispatch
        //let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
        //    &command_buffer_allocator,
        //    queue.queue_family_index(),
        //    vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        //)
        //.map_err(|_| "could not create command buffer builder")?;

        //builder
        //    .begin_render_pass(
        //        vulkano::command_buffer::RenderPassBeginInfo {
        //            clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
        //            ..vulkano::command_buffer::RenderPassBeginInfo::framebuffer(framebuffer.clone())
        //        },
        //        vulkano::command_buffer::SubpassContents::Inline,
        //    )
        //    .map_err(|_| "could not begin render pass")?
        //    .bind_pipeline_graphics(pipeline.clone())
        //    .bind_vertex_buffers(0, vertex_buffer.clone())
        //    .draw(3, 1, 0, 0)
        //    .map_err(|_| "could not draw pipeline")?
        //    .end_render_pass()
        //    .map_err(|_| "could not end render pass")?
        //    .copy_image_to_buffer(
        //        vulkano::command_buffer::CopyImageToBufferInfo::image_buffer(image, buf.clone()),
        //    )
        //    .map_err(|_| "could not copy image to buffer")?;

        //let command_buffer = builder
        //    .build()
        //    .map_err(|_| "could not build command buffer")?;
        //let future = vulkano::sync::now(device.clone())
        //    .then_execute(queue.clone(), command_buffer)
        //    .map_err(|_| "could not execute command buffer")?
        //    .then_signal_fence_and_flush()
        //    .map_err(|_| "could not signal fence and flush")?;
        //future.wait(None).map_err(|_| "could not wait on future")?;

        //// dont forget to delete image from toml
        //let buffer_content = buf.read().map_err(|_| "could not read buffer")?;
        //let image =
        //    image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..])
        //        .ok_or("could not create image buffer")?;
        //image
        //    .save("image.png")
        //    .map_err(|_| "could not safe image")?;

        //ris_log::debug!("i just rendered a triangle");

        //Ok(Self {
        //    _device: device,
        //    _queue: queue,
        //})

        Ok(Self {})
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        _logic: &LogicData,
        _frame: &FrameData,
    ) -> GameloopState {
        GameloopState::WantsToContinue
    }
}
