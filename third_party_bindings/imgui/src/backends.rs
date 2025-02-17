use std::ffi::CStr;
use std::ffi::CString;

use ash::vk;
use sdl2::video::Window;
use sdl2_sys::SDL_Event;

use crate::bindings::backends::imgui_impl_sdl2;
use crate::bindings::backends::imgui_impl_vulkan;

pub struct ImGuiBackends;

#[derive(Debug)]
pub struct InitErr {
    message: String,
}

impl std::fmt::Display for InitErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "init failed: \"{}\"", self.message)
    }
}

impl std::error::Error for InitErr {}

impl InitErr {
    fn new<T>(message: impl AsRef<str>) -> Result<T, Self> {
        Err(InitErr{
            message: message.as_ref().to_string(),
        })
    }
}

impl Drop for ImGuiBackends {
    fn drop(&mut self) {
        unsafe {
            imgui_impl_vulkan::ImGui_ImplVulkan_Shutdown();
            imgui_impl_sdl2::ImGui_ImplSDL2_Shutdown();

            // cleanup vulkan window
            // cleanup vulkan
        }
    }
}

impl ImGuiBackends {
    /// Safety: `window` must outlive `ImGuiBackends`
    pub unsafe fn init(
        window: &Window,
        instance: &ash::Instance,
        surface: &vk::SurfaceKHR,
    ) -> Result<Self, InitErr> {
        // setup sdl2
        let window_ptr = window.raw() as *mut imgui_impl_sdl2::SDL_Window;

        // setup vulkan
        let instance_handle = vk::Handle::as_raw(instance.handle());
        //let instance = &mut instance.handle() 
        //    as *mut ash::vk::Instance
        //    as *mut imgui_impl_vulkan::VkInstance_T;
        let instance = instance_handle as imgui_impl_vulkan::VkInstance;

        // select physical device (gpu)
        let physical_device = imgui_impl_vulkan::ImGui_ImplVulkanH_SelectPhysicalDevice(instance);
        if physical_device.is_null() {
            return InitErr::new("physical_device is null");
        }

        // select graphics queue family
        let queue_family = imgui_impl_vulkan::ImGui_ImplVulkanH_SelectQueueFamilyIndex(physical_device);
        if queue_family == u32::MAX {
            return InitErr::new("queue_family was -1");
        }

        // create logical device (with 1 queue)
        let allocator = std::ptr::null_mut();

        let (device, queue) = {
            //let Ok(device_extension_swapchain) = CString::new("VK_KHR_swapchain") else {
            //    return InitErr::new("failed to create swapchain extension string");
            //};
            //let device_extensions = [device_extension_swapchain.as_c_str().as_ptr()];

            let device_extensions = [std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0").as_ptr()];

            let mut properties_count = 0;
            let mut properties = Vec::new();
            imgui_impl_vulkan::vkEnumerateDeviceExtensionProperties(
                physical_device,
                std::ptr::null(),
                &mut properties_count,
                std::ptr::null_mut(),
            );
            properties.resize(
                properties_count as usize,
                imgui_impl_vulkan::VkExtensionProperties{
                    extensionName: [0; 256usize],
                    specVersion: 0,
                },
            );
            imgui_impl_vulkan::vkEnumerateDeviceExtensionProperties(
                physical_device,
                std::ptr::null(),
                &mut properties_count,
                properties.as_mut_ptr(),
            );

            let queue_priority = [1.0];
            let queue_info = [imgui_impl_vulkan::VkDeviceQueueCreateInfo{
                sType: imgui_impl_vulkan::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                queueFamilyIndex: queue_family,
                queueCount: queue_priority.len() as u32,
                pQueuePriorities: queue_priority.as_ptr(),
            }];

            let device_info = imgui_impl_vulkan::VkDeviceCreateInfo {
                sType: imgui_impl_vulkan::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                queueCreateInfoCount: queue_info.len() as u32,
                pQueueCreateInfos: queue_info.as_ptr(),
                enabledLayerCount: 0,
                ppEnabledLayerNames: std::ptr::null(),
                enabledExtensionCount: device_extensions.len() as u32,
                ppEnabledExtensionNames: device_extensions.as_ptr(),
                pEnabledFeatures: std::ptr::null(),
            };

            //let device = std::ptr::null_mut();
            let mut device = std::mem::MaybeUninit::uninit();
            let err = imgui_impl_vulkan::vkCreateDevice(
                physical_device,
                &device_info,
                allocator,
                device.as_mut_ptr(),
            );
            if err != imgui_impl_vulkan::VkResult_VK_SUCCESS {
                return InitErr::new(format!("failed to create device: {}", err));
            }

            let device = device.assume_init();

            let mut queue = std::mem::MaybeUninit::uninit();
            imgui_impl_vulkan::vkGetDeviceQueue(
                device,
                queue_family,
                0,
                queue.as_mut_ptr(),
            );

            (device, queue.assume_init())
        };

        // create descriptor pool
        // if you wish to load e.g. additional textures you may need to alter pools sizes and
        // maxSets
        let descriptor_pool = {
            let pool_sizes = [imgui_impl_vulkan::VkDescriptorPoolSize{
                type_: imgui_impl_vulkan::VkDescriptorType_VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                descriptorCount: imgui_impl_vulkan::IMGUI_IMPL_VULKAN_MINIMUM_IMAGE_SAMPLER_POOL_SIZE,
            }];

            let mut max_sets = 0;
            for pool_size in pool_sizes.iter() {
                max_sets += pool_size.descriptorCount;
            }

            let pool_info = imgui_impl_vulkan::VkDescriptorPoolCreateInfo {
                sType: imgui_impl_vulkan::VkStructureType_VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: imgui_impl_vulkan::VkDescriptorPoolCreateFlagBits_VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT as u32,
                maxSets: max_sets,
                poolSizeCount: pool_sizes.len() as u32,
                pPoolSizes: pool_sizes.as_ptr(),
            };
            let mut descriptor_pool = std::mem::MaybeUninit::uninit();
            let err = imgui_impl_vulkan::vkCreateDescriptorPool(
                device,
                &pool_info,
                allocator,
                descriptor_pool.as_mut_ptr(),
            );

            if err != imgui_impl_vulkan::VkResult_VK_SUCCESS {
                return InitErr::new(format!("failed to create descriptor pool: {}", err));
            }

            descriptor_pool.assume_init()
        };

        println!("1");

        // setup vulkan window
        //let Ok(surface) = window.vulkan_create_surface(instance_handle as usize) else {
        //    return InitErr::new("failed to create vulkan surface");
        //};

        println!("2");

        let mut wd = unsafe {
            let mut wd = std::mem::MaybeUninit::<imgui_impl_vulkan::ImGui_ImplVulkanH_Window>::uninit();
            std::ptr::write_bytes(wd.as_mut_ptr(), 0, 1);
            wd.assume_init()
        };
        wd.PresentMode = !0;
        wd.ClearEnable = true;

        wd.Surface = surface as imgui_impl_vulkan::VkSurfaceKHR;

        println!("3");

        // check for wsi support
        let mut res = imgui_impl_vulkan::VK_FALSE;
        imgui_impl_vulkan::vkGetPhysicalDeviceSurfaceSupportKHR(
            physical_device,
            queue_family,
            wd.Surface,
            &mut res
        );
        if res != imgui_impl_vulkan::VK_TRUE {
            return InitErr::new("error no wsi support on physical device 0");
        }

        println!("4");

        // select surface format
        let request_surface_image_format = [
            imgui_impl_vulkan::VkFormat_VK_FORMAT_B8G8R8A8_UNORM,
            imgui_impl_vulkan::VkFormat_VK_FORMAT_R8G8B8A8_UNORM,
            imgui_impl_vulkan::VkFormat_VK_FORMAT_B8G8R8_UNORM,
            imgui_impl_vulkan::VkFormat_VK_FORMAT_R8G8B8_UNORM,
        ];
        let request_surface_color_space = imgui_impl_vulkan::VkColorSpaceKHR_VK_COLORSPACE_SRGB_NONLINEAR_KHR;
        wd.SurfaceFormat = imgui_impl_vulkan::ImGui_ImplVulkanH_SelectSurfaceFormat(
            physical_device,
            wd.Surface,
            request_surface_image_format.as_ptr(),
            request_surface_image_format.len() as i32,
            request_surface_color_space,
        );

        println!("5");

        // select present mode
        let present_modes = [imgui_impl_vulkan::VkPresentModeKHR_VK_PRESENT_MODE_IMMEDIATE_KHR];
        wd.PresentMode = imgui_impl_vulkan::ImGui_ImplVulkanH_SelectPresentMode(
            physical_device,
            wd.Surface,
            present_modes.as_ptr(),
            present_modes.len() as i32,
        );

        println!("6 {}", wd.PresentMode);

        // create swapchain, renderpass, framebuffer, etc.
        let (w, h) = window.size();
        let min_image_count = 2;
        imgui_impl_vulkan::ImGui_ImplVulkanH_CreateOrResizeWindow(
            instance,
            physical_device,
            device,
            &mut wd,
            queue_family,
            allocator,
            w as i32,
            h as i32,
            min_image_count,
        );

        println!("7");

        // setup backens
        let pipeline_rendering_create_info = unsafe {
            let mut pipeline_rendering_create_info = std::mem::MaybeUninit::<imgui_impl_vulkan::VkPipelineRenderingCreateInfoKHR>::uninit();
            std::ptr::write_bytes(pipeline_rendering_create_info.as_mut_ptr(), 0, 1);
            pipeline_rendering_create_info.assume_init()
        };

        println!("8");

        let mut vulkan_init_info = imgui_impl_vulkan::ImGui_ImplVulkan_InitInfo {
            Instance: instance,
            PhysicalDevice: physical_device,
            Device: device,
            QueueFamily: queue_family,
            Queue: queue,
            PipelineCache: std::ptr::null_mut(),
            DescriptorPool: descriptor_pool,
            RenderPass: wd.RenderPass,
            Subpass: 0,
            MinImageCount: min_image_count,
            ImageCount: wd.ImageCount,
            MSAASamples: imgui_impl_vulkan::VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
            Allocator: allocator,
            CheckVkResultFn: Some(check_vk_result),

            DescriptorPoolSize: 0,
            UseDynamicRendering: false,
            PipelineRenderingCreateInfo: pipeline_rendering_create_info,
            MinAllocationSize: 0,
        };

        let vulkan_init_info_ptr = (&mut vulkan_init_info) as *mut imgui_impl_vulkan::ImGui_ImplVulkan_InitInfo;

        unsafe {
            imgui_impl_sdl2::ImGui_ImplSDL2_InitForVulkan(window_ptr);
            imgui_impl_vulkan::ImGui_ImplVulkan_Init(vulkan_init_info_ptr);
        }

        Ok(Self)
    }

    pub unsafe fn process_event(&mut self, event: &SDL_Event) -> bool {
        let ptr = event as *const SDL_Event as *const imgui_impl_sdl2::SDL_Event;
        imgui_impl_sdl2::ImGui_ImplSDL2_ProcessEvent(ptr)
    }

    pub fn new_frame(&mut self) {
        unsafe {
            imgui_impl_vulkan::ImGui_NewFrame();
            imgui_impl_sdl2::ImGui_NewFrame();
            crate::bindings::imgui::ImGui_NewFrame();
        }
    }
}

extern "C" fn check_vk_result(x: i32) {
    println!("check vk result");

    if x != imgui_impl_vulkan::VkResult_VK_SUCCESS {
        println!("ERROR: ImGui Vulkan backend check was unsuccessful. VkResult: {}", x)
    }
}
