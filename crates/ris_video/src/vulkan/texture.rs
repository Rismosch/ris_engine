use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

pub struct Texture {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
}

impl Texture {
    pub fn alloc(
        device: &ash::Device,
        width: u32,
        height: u32,
        format: vk::Format,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<Self> {
        let image_create_info = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ImageCreateFlags::empty(),
            image_type: vk::ImageType::TYPE_2D,
            format,
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling,
            usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
            initial_layout: vk::ImageLayout::UNDEFINED,
        };

        let image = unsafe{device.create_image(&image_create_info, None)}?;

        let image_memory_requirements = unsafe{device.get_image_memory_requirements(image)};
        let memory_type_index = super::util::find_memory_type(
            image_memory_requirements.memory_type_bits,
            memory_property_flags,
            physical_device_memory_properties,
        )?.unroll()?;

        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: image_memory_requirements.size,
            memory_type_index,
        };

        let memory = unsafe{device.allocate_memory(&memory_allocate_info, None)}?;
        unsafe{device.bind_image_memory(image, memory, 0)};

        Ok(Self{
            image,
            memory,
        })
    }

    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_image(self.image, None);
            device.free_memory(self.memory, None);
        }
    }
}
