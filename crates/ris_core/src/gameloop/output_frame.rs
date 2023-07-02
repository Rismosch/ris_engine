use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, logic_data::LogicData,
    output_data::OutputData,
};
use vulkano::pipeline::Pipeline;
use vulkano::sync::GpuFuture;

pub struct OutputFrame {
    _device: std::sync::Arc<vulkano::device::Device>,
    _queue: std::sync::Arc<vulkano::device::Queue>,
}

impl OutputFrame {
    pub fn new() -> Result<Self, String> {
        // init vulkan
        let library = vulkano::VulkanLibrary::new().map_err(|_| "no local Vulkan library/DLL")?;
        let instance = vulkano::instance::Instance::new(
            library,
            vulkano::instance::InstanceCreateInfo::default(),
        )
        .map_err(|_| "failed to create instance")?;

        // create device
        let mut physical_devices = instance
            .enumerate_physical_devices()
            .map_err(|_| "could not enumerate devices")?;
        let physical_device = physical_devices.next().ok_or("no devices available")?;

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(vulkano::device::QueueFlags::GRAPHICS)
            })
            .ok_or("could not find a graphical queue family")?
            as u32;

        let (device, mut queues) = vulkano::device::Device::new(
            physical_device,
            vulkano::device::DeviceCreateInfo {
                queue_create_infos: vec![vulkano::device::QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .map_err(|_| "failed to create device")?;

        let queue = queues.next().ok_or("no queues available")?;

        // allocators
        let memory_allocator =
            vulkano::memory::allocator::StandardMemoryAllocator::new_default(device.clone());
        let descriptor_set_allocator =
            vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator::new(device.clone());
        let command_buffer_allocator = vulkano::command_buffer::allocator::StandardCommandBufferAllocator::new(
            device.clone(),
            vulkano::command_buffer::allocator::StandardCommandBufferAllocatorCreateInfo::default()
        );

        // shader
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

        let compiler = shaderc::Compiler::new().ok_or("could not initialize shaderc compiler")?;
        let options =
            shaderc::CompileOptions::new().ok_or("could not initialize shaderc options")?;

        let binary_result = compiler
            .compile_into_spirv(
                source,
                shaderc::ShaderKind::Compute,
                "shader.glsl",
                "main",
                Some(&options),
            )
            .map_err(|_| "could not compile shader")?;

        let words: &[u32] = binary_result.as_binary();
        let shader_module =
            unsafe { vulkano::shader::ShaderModule::from_words(device.clone(), words) }
                .map_err(|_| "could not load shader module")?;
        let entry_point = shader_module.entry_point("main").unwrap();

        // data buffer
        let data_iter = 0..65536u32;
        let data_buffer = vulkano::buffer::Buffer::from_iter(
            &memory_allocator,
            vulkano::buffer::BufferCreateInfo {
                usage: vulkano::buffer::BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::Upload,
                ..Default::default()
            },
            data_iter,
        )
        .map_err(|_| "failed to create data buffer")?;

        // compute pipeline
        let compute_pipeline =
            vulkano::pipeline::ComputePipeline::new(device.clone(), entry_point, &(), None, |_| {})
                .map_err(|_| "failed to create compute pipeline")?;

        let pipeline_layout = compute_pipeline.layout();
        let descriptor_set_layouts = pipeline_layout.set_layouts();

        let descriptor_set_layout_index = 0;
        let descriptor_set_layout = descriptor_set_layouts
            .get(descriptor_set_layout_index)
            .ok_or("no descriptor layouts available")?;
        let descriptor_set = vulkano::descriptor_set::PersistentDescriptorSet::new(
            &descriptor_set_allocator,
            descriptor_set_layout.clone(),
            [vulkano::descriptor_set::WriteDescriptorSet::buffer(
                0,
                data_buffer.clone(),
            )],
        )
        .map_err(|_| "could not create descriptor set")?;

        // command buffer
        let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .map_err(|_| "could not create builder")?;

        let work_group_counts = [1024, 1, 1];

        builder
            .bind_pipeline_compute(compute_pipeline.clone())
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Compute,
                compute_pipeline.layout().clone(),
                descriptor_set_layout_index as u32,
                descriptor_set,
            )
            .dispatch(work_group_counts)
            .map_err(|_| "could not execute pipeline")?;

        // execute
        let command_buffer = builder
            .build()
            .map_err(|_| "could not build command buffer")?;
        let future = vulkano::sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .map_err(|_| "could not execute command buffer")?
            .then_signal_fence_and_flush()
            .map_err(|_| "could not signal fence and flush")?;
        future.wait(None).unwrap();

        let content = data_buffer.read().map_err(|_| "could not read buffer")?;
        ris_log::debug!("hello: {:?}", &*content);

        Ok(Self {
            _device: device,
            _queue: queue,
        })
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
