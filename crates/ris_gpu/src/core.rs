use std::ffi::CStr;
use std::ffi::CString;

use ash::vk;
use sdl2::video::Window;

use ris_error::Extensions;
use ris_error::RisResult;

use super::debug::Debugger;
use super::suitable_device::SuitableDevice;
use super::swapchain::Swapchain;
use super::swapchain::SwapchainCreateInfo;

pub struct VulkanCore {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debugger: Debugger,
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
    pub suitable_device: SuitableDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub transient_command_pool: vk::CommandPool,
    pub swapchain: Swapchain,
}

impl VulkanCore {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&mut self) {
        ris_log::debug!("dropping vulkan core...");

        self.swapchain.free(&self.device);

        self.device
            .destroy_command_pool(self.transient_command_pool, None);

        self.device.destroy_device(None);
        self.surface_loader.destroy_surface(self.surface, None);

        self.debugger.free();

        self.instance.destroy_instance(None);

        ris_log::info!("vulkan core dropped!");
    }

    pub fn alloc(application_name: &str, window: &Window) -> RisResult<Self> {
        let entry = unsafe { ash::Entry::load() }?;

        // instance extensions
        let mut count = 0;
        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(
                window.raw(),
                &mut count,
                std::ptr::null_mut(),
            )
        } == sdl2_sys::SDL_bool::SDL_FALSE
        {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        let mut instance_extensions = vec![std::ptr::null(); count as usize];

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
        let available_layers = super::debug::get_layers(&entry, &mut instance_extensions)?;
        let mut available_layers_ptrs = Vec::with_capacity(available_layers.len());
        for layer in available_layers.iter() {
            let ptr = layer.as_ptr();
            available_layers_ptrs.push(ptr);
        }

        let mut log_message = format!("Vulkan Instance Extensions: {}", instance_extensions.len());
        for extension in instance_extensions.iter() {
            let extension_name = unsafe { CStr::from_ptr(*extension) }.to_str()?;
            log_message.push_str(&format!("\n\t- {}", extension_name));
        }
        ris_log::trace!("{}", log_message);

        // instance
        let cstring_application_name = CString::new(application_name.to_string())?;
        let vk_app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: cstring_application_name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: cstring_application_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 0, 92),
        };

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &vk_app_info,
            pp_enabled_layer_names: available_layers_ptrs.as_ptr(),
            enabled_layer_count: available_layers_ptrs.len() as u32,
            pp_enabled_extension_names: instance_extensions.as_ptr(),
            enabled_extension_count: instance_extensions.len() as u32,
        };

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let debugger = Debugger::alloc(&entry, &instance)?;

        // surface
        let instance_handle = vk::Handle::as_raw(instance.handle());
        let surface_raw = window
            .vulkan_create_surface(instance_handle as usize)
            .into_ris_error()?;
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

        let mut unique_queue_families = std::collections::HashSet::new();
        unique_queue_families.insert(suitable_device.graphics_queue_family);
        unique_queue_families.insert(suitable_device.present_queue_family);

        ris_log::debug!("chosen queue families: {:?}", unique_queue_families);

        let queue_priorities = [1.0_f32];
        let mut queue_create_infos = Vec::new();
        for queue_family in unique_queue_families {
            let queue_create_info = vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: std::ptr::null(),
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

        let mut enabled_extension_names = Vec::with_capacity(suitable_device.extensions.len());
        for extension_name in suitable_device.extensions.iter() {
            enabled_extension_names.push(extension_name.as_ptr());
        }

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            pp_enabled_extension_names: enabled_extension_names.as_ptr(),
            enabled_extension_count: enabled_extension_names.len() as u32,
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

        if graphics_queue == present_queue {
            debugger.set_name(&device, graphics_queue, "graphics_present_queue")?;
        } else {
            debugger.set_name(&device, graphics_queue, "graphics_queue")?;
            debugger.set_name(&device, present_queue, "present_queue")?;
        }

        // command pool
        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandPoolCreateFlags::TRANSIENT,
            queue_family_index: suitable_device.graphics_queue_family,
        };
        let transient_command_pool =
            unsafe { device.create_command_pool(&command_pool_create_info, None) }?;

        // swapchain
        let swapchain = Swapchain::alloc(SwapchainCreateInfo {
            instance: &instance,
            surface_loader: &surface_loader,
            surface: &surface,
            suitable_device: &suitable_device,
            device: &device,
            graphics_queue,
            transient_command_pool,
            window_drawable_size: window.vulkan_drawable_size(),
        })?;
        debugger.set_name(
            &device,
            swapchain.swapchain,
            format!("swapchain_gen_{}", swapchain.generation),
        )?;
        ris_log::trace!("swapchain created! entries: {}", swapchain.entries.len());

        // renderer
        Ok(Self {
            entry,
            instance,
            debugger,
            surface_loader,
            surface,
            suitable_device,
            device,
            graphics_queue,
            present_queue,
            transient_command_pool,
            swapchain,
        })
    }

    pub fn recreate_swapchain(&mut self, window_drawable_size: (u32, u32)) -> RisResult<()> {
        let Self {
            instance,
            debugger,
            surface_loader,
            surface,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            swapchain,
            ..
        } = self;

        ris_log::trace!("recreating swapchain...");

        let previous_generation = swapchain.generation;

        unsafe {
            device.device_wait_idle()?;
            swapchain.free(device);
            *swapchain = Swapchain::alloc(SwapchainCreateInfo {
                instance,
                surface_loader,
                surface,
                suitable_device,
                device,
                graphics_queue: *graphics_queue,
                transient_command_pool: *transient_command_pool,
                window_drawable_size,
            })?;
        }

        swapchain.generation = previous_generation + 1;
        debugger.set_name(
            device,
            swapchain.swapchain,
            format!("swapchain_gen_{}", swapchain.generation),
        )?;

        ris_log::trace!(
            "swapchain recreated! gen: {} entries: {}",
            swapchain.generation,
            swapchain.entries.len(),
        );
        Ok(())
    }
}
