use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ash::vk;
use sdl2::video::Window;

use ris_asset::codecs::qoi;
use ris_asset::RisGodAsset;
use ris_data::info::app_info::AppInfo;
use ris_error::Extensions;
use ris_error::RisResult;

use super::buffer::Buffer;
use super::suitable_device::SuitableDevice;
use super::swapchain::Swapchain;
use super::swapchain::SwapchainCreateInfo;
use super::texture::Texture;
use super::texture::TextureCreateInfo;
use super::transient_command::TransientCommandSync;

pub struct VulkanCore {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_utils: Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
    pub suitable_device: SuitableDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    //pub descriptor_set_layout: vk::DescriptorSetLayout,
    //pub descriptor_pool: vk::DescriptorPool,
    pub command_pool: vk::CommandPool,
    pub transient_command_pool: vk::CommandPool,
    //pub texture: Texture,
    //pub vertex_buffer: Buffer,
    //pub index_buffer: Buffer,
    pub swapchain: Swapchain,
}

impl Drop for VulkanCore {
    fn drop(&mut self) {
        ris_log::debug!("dropping vulkan core...");

        unsafe {
            self.swapchain.free(&self.device, self.command_pool);

            self.device
                .destroy_command_pool(self.transient_command_pool, None);
            self.device.destroy_command_pool(self.command_pool, None);

            //self.device
            //    .destroy_descriptor_pool(self.descriptor_pool, None);
            //self.device
            //    .destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            //self.index_buffer.free(&self.device);
            //self.vertex_buffer.free(&self.device);
            //self.texture.free(&self.device);

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            if let Some((debug_utils, debug_utils_messenger)) = self.debug_utils.take() {
                debug_utils.destroy_debug_utils_messenger(debug_utils_messenger, None);
            }

            self.instance.destroy_instance(None);
        }

        ris_log::info!("vulkan core dropped!");
    }
}

impl VulkanCore {
    pub fn initialize(
        app_info: &AppInfo,
        window: &Window,
        god_asset: &RisGodAsset,
    ) -> RisResult<Self> {
        let entry = unsafe { ash::Entry::load() }?;

        // instance extensions
        let mut count = 0;
        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, ptr::null_mut())
        } == sdl2_sys::SDL_bool::SDL_FALSE
        {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        let mut instance_extensions = vec![ptr::null(); count as usize];

        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(
                window.raw(),
                &mut count,
                instance_extensions.as_mut_ptr(),
            )
        } == sdl2_sys::SDL_bool::SDL_FALSE
        {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        // validation layers
        let available_layers =
            super::layers::add_validation_layer(&entry, &mut instance_extensions)?;

        let mut log_message = format!("Vulkan Instance Extensions: {}", instance_extensions.len());
        for extension in instance_extensions.iter() {
            let extension_name = unsafe { CStr::from_ptr(*extension) }.to_str()?;
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

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let debug_utils = super::layers::setup_debugging(&entry, &instance)?;

        // surface
        let instance_handle = vk::Handle::as_raw(instance.handle());
        let surface_raw = window
            .vulkan_create_surface(instance_handle as usize)
            .unroll()?;
        let surface: vk::SurfaceKHR = vk::Handle::from_raw(surface_raw);
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        // suitable devices
        let suitable_devices = SuitableDevice::query(&instance, &surface_loader, surface)?;

        // logical device
        let Some(suitable_device) = suitable_devices.into_iter().min_by_key(|x| x.suitability)
        else {
            return ris_error::new_result!(
                "no suitable hardware found to initialize vulkan renderer"
            );
        };

        ris_log::info!("chosen Vulkan Physical Device: {}", suitable_device.name);

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };
        let physical_device_properties =
            unsafe { instance.get_physical_device_properties(suitable_device.physical_device) };

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
            geometry_shader: vk::TRUE,
            sampler_anisotropy: vk::TRUE,
            ..Default::default()
        };

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            //pp_enabled_layer_names: available_layers.1,
            //enabled_layer_count: available_layers.0,
            pp_enabled_extension_names: super::REQUIRED_DEVICE_EXTENSIONS.as_ptr(),
            enabled_extension_count: super::REQUIRED_DEVICE_EXTENSIONS.len() as u32,
            p_enabled_features: &physical_device_features,
            ..Default::default()
        };

        let device = unsafe {
            instance.create_device(suitable_device.physical_device, &device_create_info, None)
        }?;
        let graphics_queue =
            unsafe { device.get_device_queue(suitable_device.graphics_queue_family, 0) };
        let present_queue =
            unsafe { device.get_device_queue(suitable_device.present_queue_family, 0) };

        //// descriptor set layout
        //let ubo_layout_bindings = [
        //    vk::DescriptorSetLayoutBinding {
        //        binding: 0,
        //        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
        //        descriptor_count: 1,
        //        stage_flags: vk::ShaderStageFlags::VERTEX,
        //        p_immutable_samplers: ptr::null(),
        //    },
        //    vk::DescriptorSetLayoutBinding {
        //        binding: 1,
        //        descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        //        descriptor_count: 1,
        //        stage_flags: vk::ShaderStageFlags::FRAGMENT,
        //        p_immutable_samplers: ptr::null(),
        //    },
        //];

        //let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
        //    s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        //    p_next: ptr::null(),
        //    flags: vk::DescriptorSetLayoutCreateFlags::empty(),
        //    binding_count: ubo_layout_bindings.len() as u32,
        //    p_bindings: ubo_layout_bindings.as_ptr(),
        //};

        //let descriptor_set_layout = unsafe {
        //    device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
        //}?;

        // command pool
        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            queue_family_index: suitable_device.graphics_queue_family,
        };
        let command_pool = unsafe { device.create_command_pool(&command_pool_create_info, None) }?;

        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::TRANSIENT,
            queue_family_index: suitable_device.graphics_queue_family,
        };
        let transient_command_pool =
            unsafe { device.create_command_pool(&command_pool_create_info, None) }?;

        //// texture
        //let texture_asset_id = god_asset.texture.clone();
        //let content = ris_asset::load_async(texture_asset_id.clone()).wait(None)??;
        //let (pixels, desc) = qoi::decode(&content, None)?;

        //let pixels_rgba = match desc.channels {
        //    qoi::Channels::RGB => {
        //        ris_log::trace!(
        //            "adding alpha channel to texture asset... {:?}",
        //            texture_asset_id
        //        );

        //        ris_error::assert!(pixels.len() % 3 == 0)?;
        //        let pixels_rgba_len = (pixels.len() * 4) / 3;
        //        let mut pixels_rgba = Vec::with_capacity(pixels_rgba_len);

        //        for chunk in pixels.chunks_exact(3) {
        //            let r = chunk[0];
        //            let g = chunk[1];
        //            let b = chunk[2];
        //            let a = u8::MAX;

        //            pixels_rgba.push(r);
        //            pixels_rgba.push(g);
        //            pixels_rgba.push(b);
        //            pixels_rgba.push(a);
        //        }

        //        ris_log::trace!(
        //            "added alpha channel to texture asset! {:?}",
        //            texture_asset_id
        //        );

        //        pixels_rgba
        //    }
        //    qoi::Channels::RGBA => pixels,
        //};

        //let texture = unsafe {
        //    Texture::alloc(TextureCreateInfo {
        //        device: &device,
        //        queue: graphics_queue,
        //        transient_command_pool,
        //        physical_device_memory_properties,
        //        physical_device_properties,
        //        width: desc.width,
        //        height: desc.height,
        //        pixels_rgba: &pixels_rgba,
        //    })
        //}?;

        //// descriptor pool
        //let descriptor_pool_sizes = [
        //    vk::DescriptorPoolSize {
        //        ty: vk::DescriptorType::UNIFORM_BUFFER,
        //        descriptor_count: swapchain_entry_count as u32,
        //    },
        //    vk::DescriptorPoolSize {
        //        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        //        descriptor_count: swapchain_entry_count as u32,
        //    },
        //];

        //let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
        //    s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
        //    p_next: ptr::null(),
        //    flags: vk::DescriptorPoolCreateFlags::empty(),
        //    max_sets: swapchain_entry_count as u32,
        //    pool_size_count: descriptor_pool_sizes.len() as u32,
        //    p_pool_sizes: descriptor_pool_sizes.as_ptr(),
        //};

        //let descriptor_pool =
        //    unsafe { device.create_descriptor_pool(&descriptor_pool_create_info, None) }?;

        // swapchain
        let swapchain = unsafe {
            Swapchain::alloc(SwapchainCreateInfo {
                instance: &instance,
                suitable_device: &suitable_device,
                device: &device,
                graphics_queue,
                command_pool,
                transient_command_pool,
                surface_loader: &surface_loader,
                surface: &surface,
                window_drawable_size: window.vulkan_drawable_size(),
            })
        }?;

        // renderer
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
            command_pool,
            transient_command_pool,
            swapchain,
        })
    }

    pub fn recreate_swapchain(
        &mut self,
        window_drawable_size: (u32, u32),
    ) -> RisResult<()> {
        let Self {
            instance,
            surface_loader,
            surface,
            suitable_device,
            device,
            graphics_queue,
            command_pool,
            transient_command_pool,
            ..
        } = self;

        ris_log::trace!("recreating swapchain...");

        unsafe {
            device.device_wait_idle()?;
            self.swapchain.free(device, *command_pool);
            self.swapchain = Swapchain::alloc(SwapchainCreateInfo {
                instance,
                suitable_device,
                device,
                graphics_queue: *graphics_queue,
                command_pool: *command_pool,
                transient_command_pool: *transient_command_pool,
                surface_loader,
                surface,
                window_drawable_size,
            })?;
        }

        ris_log::trace!("swapchain recreated!");

        Ok(())
    }
}
