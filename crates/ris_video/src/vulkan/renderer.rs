use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use ash::vk;
use sdl2::video::Window;

use ris_asset::AssetId;
use ris_asset::loader::scenes_loader::Scenes;
use ris_data::info::app_info::AppInfo;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::color::Rgb;
use ris_math::matrix::Mat4;
use ris_math::vector::Vec3;
use ris_math::vector::Vec2;

use super::buffer::Buffer;
use super::frame_in_flight::FrameInFlight;
use super::suitable_device::SuitableDevice;
use super::surface_details::SurfaceDetails;
use super::swapchain_objects::SwapchainObjects;
use super::texture::Texture;
use super::transient_command::TransientCommand;
use super::uniform_buffer_object::UniformBufferObject;
use super::util;
use super::vertex::Vertex;

pub struct Renderer {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_utils: Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
    pub suitable_device: SuitableDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub descriptor_pool: vk::DescriptorPool,
    pub swapchain_objects: SwapchainObjects,
    pub command_pool: vk::CommandPool,
    //pub texture_image: vk::Image,
    //pub texture_image_memory: vk::DeviceMemory,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub transient_command_pool: vk::CommandPool,
    pub frames_in_flight: Vec<FrameInFlight>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        ris_log::debug!("dropping renderer...");

        unsafe {
            self.device.device_wait_idle();

            for frame_in_flight in self.frames_in_flight.iter() {
                self.device.destroy_fence(frame_in_flight.in_flight_fence, None);
                self.device.destroy_semaphore(frame_in_flight.render_finished_semaphore, None);
                self.device.destroy_semaphore(frame_in_flight.image_available_semaphore, None);

                frame_in_flight.uniform_buffer.free(&self.device);
            }

            self.device.destroy_command_pool(self.transient_command_pool, None);
            self.device.destroy_command_pool(self.command_pool, None);

            self.swapchain_objects.cleanup(&self.device);

            self.device.destroy_descriptor_pool(self.descriptor_pool, None);
            self.device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.vertex_buffer.free(&self.device);
            self.index_buffer.free(&self.device);

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            if let Some((debug_utils, debug_utils_messenger)) = self.debug_utils.take() {
                debug_utils.destroy_debug_utils_messenger(debug_utils_messenger, None);
            }

            self.instance.destroy_instance(None);
        }

        ris_log::info!("renderer dropped!");
    }
}

impl Renderer {
    pub fn initialize(app_info: &AppInfo, window: &Window, scenes: Scenes) -> RisResult<Self> {
        let entry = unsafe {ash::Entry::load()}?;
        
        // instance extensions
        let mut count = 0;
        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, ptr::null_mut())
        } == sdl2_sys::SDL_bool::SDL_FALSE {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        let mut instance_extensions = vec![ptr::null(); count as usize];

        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, instance_extensions.as_mut_ptr())
        } == sdl2_sys::SDL_bool::SDL_FALSE {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        // validation layers
        let available_layers = super::layers::add_validation_layer(
            &entry,
            &mut instance_extensions,
        )?;

        let mut log_message = format!("Vulkan Instance Extensions: {}", instance_extensions.len());
        for extension in instance_extensions.iter() {
            let extension_name = unsafe{CStr::from_ptr(*extension)}.to_str()?;
            log_message.push_str(&format!("\n\t- {}", extension_name));
        }
        ris_log::trace!("{}", log_message);

        // instance
        let name = CString::new(app_info.package.name.clone())?;
        let vk_app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 0, 92),
        };

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &vk_app_info,
            pp_enabled_layer_names: available_layers.1,
            enabled_layer_count: available_layers.0,
            pp_enabled_extension_names: instance_extensions.as_ptr(),
            enabled_extension_count: instance_extensions.len() as u32,
        };

        let instance = unsafe {
            entry.create_instance(&create_info, None)?
        };

        let debug_utils = super::layers::setup_debugging(
            &entry,
            &instance,
        )?;

        // surface
        let instance_handle = vk::Handle::as_raw(instance.handle());
        let surface_raw = window.vulkan_create_surface(instance_handle as usize).unroll()?;
        let surface: vk::SurfaceKHR = vk::Handle::from_raw(surface_raw);
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        // suitable devices
        let suitable_devices = SuitableDevice::query(
            &instance,
            &surface_loader,
            surface,
        )?;

        // logical device
        let Some(suitable_device) = suitable_devices
            .into_iter()
            .min_by_key(|x| x.suitability) else {
            return ris_error::new_result!("no suitable hardware found to initialize vulkan renderer");
        };

        let mut unique_queue_families = std::collections::HashSet::new();
        unique_queue_families.insert(suitable_device.graphics_queue_family);
        unique_queue_families.insert(suitable_device.present_queue_family);

        ris_log::debug!("chosen queue families: {:?}", unique_queue_families);

        let queue_priorities = [1.0_f32];
        let mut queue_create_infos = Vec::new();
        for queue_family in unique_queue_families {
            let queue_create_info = vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: queue_family,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: queue_priorities.len() as u32,
            };
            queue_create_infos.push(queue_create_info);
        }

        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            pp_enabled_layer_names: available_layers.1,
            enabled_layer_count: available_layers.0,
            pp_enabled_extension_names: super::REQUIRED_DEVICE_EXTENSIONS.as_ptr(),
            enabled_extension_count: super::REQUIRED_DEVICE_EXTENSIONS.len() as u32,
            p_enabled_features: &physical_device_features,
        };

        let device = unsafe {instance.create_device(suitable_device.physical_device, &device_create_info, None)}?;
        let graphics_queue = unsafe{device.get_device_queue(suitable_device.graphics_queue_family, 0)};
        let present_queue = unsafe{device.get_device_queue(suitable_device.present_queue_family, 0)};

        // descriptor set layout
        let ubo_layout_bindings = [vk::DescriptorSetLayoutBinding{
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: ptr::null(),
        }];

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: ubo_layout_bindings.len() as u32,
            p_bindings: ubo_layout_bindings.as_ptr(),
        };

        let descriptor_set_layout = unsafe{device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)}?;

        // swap chain
        let swapchain_objects = SwapchainObjects::create(
            &instance,
            &surface_loader,
            &surface,
            &device,
            &suitable_device,
            &descriptor_set_layout,
            window.vulkan_drawable_size(),
        )?;

        // command pool
        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            queue_family_index: suitable_device.graphics_queue_family,
        };
        let command_pool = unsafe{device.create_command_pool(&command_pool_create_info, None)}?;

        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::TRANSIENT,
            queue_family_index: suitable_device.graphics_queue_family,
        };
        let transient_command_pool = unsafe{device.create_command_pool(&command_pool_create_info, None)}?;

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: super::MAX_FRAMES_IN_FLIGHT,
        };

        let command_buffers = unsafe {device.allocate_command_buffers(&command_buffer_allocate_info)}?;

        let device_memory_properties = unsafe{instance.get_physical_device_memory_properties(suitable_device.physical_device)};

        // texture image
        let texture_asset_id = ris_asset::AssetId::Directory(String::from("__imported_raw/images/ris_engine.qoi"));
        let texture_file_content = ris_asset::load_async(texture_asset_id).wait(None)??;
        let (pixels, desc) = ris_asset::codecs::qoi::decode(&texture_file_content, None)?;
        ris_log::debug!("{:?} {:?}", desc, &pixels[0..10]);

        // texture is in rgb format. add alpha channel to make it rgba
        ris_error::assert!(pixels.len() % 3 == 0)?;
        let pixels_with_alpha_length = (pixels.len() * 4) / 3;
        let mut pixels_with_alpha = Vec::with_capacity(pixels_with_alpha_length);
        for chunk in pixels.chunks_exact(3) {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            let a = u8::MAX;

            pixels_with_alpha.push(r);
            pixels_with_alpha.push(g);
            pixels_with_alpha.push(b);
            pixels_with_alpha.push(a);
        }
        let pixels = pixels_with_alpha;

        let staging_buffer = Buffer::alloc(
            &device,
            pixels.len() as vk::DeviceSize,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            &device_memory_properties,
        )?;

        staging_buffer.write(
            &device,
            &pixels,
        );

        let texture = Texture::alloc(
            &device,
            desc.width,
            desc.height,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            &device_memory_properties,
        )?;

        staging_buffer.free(&device);

        // vertex buffer
        let vertex_buffer_size = std::mem::size_of_val(&super::VERTICES) as vk::DeviceSize;

        let staging_buffer = Buffer::alloc(
            &device,
            vertex_buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            &device_memory_properties,
        )?;

        staging_buffer.write(
            &device,
            &super::VERTICES,
        );

        let vertex_buffer = Buffer::alloc(
            &device,
            vertex_buffer_size,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            &device_memory_properties,
        )?;

        staging_buffer.copy_to(
            &vertex_buffer,
            &device,
            &graphics_queue,
            &transient_command_pool,
            vertex_buffer_size,
        )?;

        staging_buffer.free(&device);

        // index buffer
        let index_buffer_size = std::mem::size_of_val(&super::INDICES) as vk::DeviceSize;
        let device_memory_properties = unsafe{instance.get_physical_device_memory_properties(suitable_device.physical_device)};

        let staging_buffer = Buffer::alloc(
            &device,
            index_buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            &device_memory_properties,
        )?;

        staging_buffer.write(
            &device,
            &super::INDICES,
        );

        let index_buffer = Buffer::alloc(
            &device,
            index_buffer_size,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            &device_memory_properties,
        )?;

        staging_buffer.copy_to(
            &index_buffer,
            &device,
            &graphics_queue,
            &transient_command_pool,
            index_buffer_size,
        )?;

        staging_buffer.free(&device);

        // descriptor pool
        let descriptor_pool_sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: command_buffers.len() as u32,
        }];

        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorPoolCreateFlags::empty(),
            max_sets: command_buffers.len() as u32,
            pool_size_count: descriptor_pool_sizes.len() as u32,
            p_pool_sizes: descriptor_pool_sizes.as_ptr(),
        };

        let descriptor_pool = unsafe{device.create_descriptor_pool(&descriptor_pool_create_info, None)}?;

        let mut descriptor_set_layouts = Vec::with_capacity(command_buffers.len());
        for _ in 0..command_buffers.len() {
            descriptor_set_layouts.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool,
            descriptor_set_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
        };

        let descriptor_sets = unsafe{device.allocate_descriptor_sets(&descriptor_set_allocate_info)}?;

        // frames in flight
        let mut frames_in_flight = Vec::with_capacity(command_buffers.len());
        for i in 0..command_buffers.len() {
            let command_buffer = command_buffers[i];
            let descriptor_set = descriptor_sets[i];

            // uniform buffer
            let uniform_buffer_size = std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize;
            ris_log::debug!("aschmo {}", uniform_buffer_size);
            let uniform_buffer = Buffer::alloc(
                &device,
                uniform_buffer_size,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                &device_memory_properties,
            )?;
            let uniform_buffer_mapped = unsafe{device.map_memory(
                uniform_buffer.memory,
                0,
                uniform_buffer_size,
                vk::MemoryMapFlags::empty()
            )}? as *mut UniformBufferObject;

            // descriptor set
            let descriptor_buffer_info = [vk::DescriptorBufferInfo {
                buffer: uniform_buffer.buffer,
                offset: 0,
                range: std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize,
            }];

            let write_descriptor_set = [vk::WriteDescriptorSet {
                s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: descriptor_set,
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: ptr::null(),
                p_buffer_info: descriptor_buffer_info.as_ptr(),
                p_texel_buffer_view: ptr::null(),
            }];

            unsafe{device.update_descriptor_sets(&write_descriptor_set, &[])};

            // synchronization objects
            let semaphore_create_info = vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::SemaphoreCreateFlags::empty(),
            };

            let fence_create_info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::FenceCreateFlags::SIGNALED,
            };

            let image_available_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
            let render_finished_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
            let in_flight_fence = unsafe{device.create_fence(&fence_create_info, None)}?;

            let frame_in_flight = FrameInFlight {
                command_buffer,
                descriptor_set,
                uniform_buffer,
                uniform_buffer_mapped,
                image_available_semaphore,
                render_finished_semaphore,
                in_flight_fence,
            };

            frames_in_flight.push(frame_in_flight);
        }

        Ok(Self {
            entry,
            instance,
            debug_utils,
            surface_loader,
            surface,
            suitable_device,
            device,
            graphics_queue,
            present_queue,
            descriptor_set_layout,
            descriptor_pool,
            swapchain_objects,
            command_pool,
            //texture_image,
            //texture_image_memory,
            vertex_buffer,
            index_buffer,
            transient_command_pool,
            frames_in_flight,
        })
    }

    pub fn recreate_swapchain(&mut self, window_size: (u32, u32)) -> RisResult<()> {
        ris_log::trace!("recreating swapchain...");

        unsafe {self.device.device_wait_idle()}?;

        self.swapchain_objects.cleanup(&self.device);
        self.swapchain_objects = SwapchainObjects::create(
            &self.instance,
            &self.surface_loader,
            &self.surface,
            &self.device,
            &self.suitable_device,
            &self.descriptor_set_layout,
            window_size,
        )?;

        ris_log::trace!("swapchain recreated!");

        Ok(())
    }
}

