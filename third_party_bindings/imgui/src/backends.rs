use std::marker::PhantomData;

use ash::vk;
use sdl2::event::EventType;
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
        }
    }
}

impl ImGuiBackends {
    /// Safety: `window` must outlive `ImGuiBackends`
    pub unsafe fn init(window: &Window, instance: &mut ash::Instance) -> Result<Self, InitErr> {
        // setup sdl2
        let window_ptr = window.raw() as *mut imgui_impl_sdl2::SDL_Window;

        // setup vulkan
        let instance = &mut instance.handle() 
            as *mut vk::Instance
            as *mut imgui_impl_vulkan::VkInstance_T;

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
        let (device, queue) = {
            let device_extensions = unsafe {
                [
                    std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0").as_ptr()
                ]
            };


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

            let device = std::ptr::null_mut();
            let err = imgui_impl_vulkan::vkCreateDevice(
                physical_device,
                &device_info,
                std::ptr::null_mut(),
                device,
            );

            if err != imgui_impl_vulkan::VkResult_VK_SUCCESS {
                return InitErr::new(format!("failed to create device: {}", err));
            }

            let queue = std::ptr::null_mut();
            imgui_impl_vulkan::vkGetDeviceQueue(
                *device,
                queue_family,
                0,
                queue,
            );

            (*device, *queue)
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
            let descriptor_pool = std::ptr::null_mut();
            let err = imgui_impl_vulkan::vkCreateDescriptorPool(
                device,
                &pool_info,
                std::ptr::null(),
                descriptor_pool,
            );

            if err != imgui_impl_vulkan::VkResult_VK_SUCCESS {
                return InitErr::new(format!("failed to create descriptor pool: {}", err));
            }

            *descriptor_pool
        };

        // setup vulkan window

        // setup backens
        let mut vulkan_init_info = imgui_impl_vulkan::ImGui_ImplVulkan_InitInfo {
            Instance: instance,
            PhysicalDevice: physical_device,
            Device: device,
            QueueFamily: queue_family,
            Queue: queue,
            //PipelineCache:,
            DescriptorPool: descriptor_pool,
            //RenderPass:,
            //Subpass: ,
            //MinImageCount:,
            //ImageCount:,
            MSAASamples: imgui_impl_vulkan::VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
            //Allocator:,
            //CheckVkResultFn:,

            //DescriptorPoolSize:,
            //UseDynamicRendering:,
            //PipelineRenderingCreateInfo:,
            //MinAllocationSize:,
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
        }
    }
}
