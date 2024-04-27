use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ash::vk;

use ris_asset::AssetId;
use ris_error::Extensions;
use ris_error::RisResult;

use super::graphics_pipeline::GraphicsPipeline;
use super::image::Image;
use super::suitable_device::SuitableDevice;
use super::surface_details::SurfaceDetails;
use super::util;
use super::vertex::Vertex;

pub struct SwapchainObjects {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_format: vk::SurfaceFormatKHR,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub graphics_pipeline: GraphicsPipeline,
    pub depth_image: Image,
    pub depth_image_view: vk::ImageView,
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl SwapchainObjects {
    pub fn cleanup(&mut self, device: &ash::Device) {
        unsafe {
            for &framebuffer in self.framebuffers.iter() {
                device.destroy_framebuffer(framebuffer, None);
            }

            self.graphics_pipeline.free(device);

            self.depth_image.free(device);
            device.destroy_image_view(self.depth_image_view, None);

            for &swapchain_image_view in self.swapchain_image_views.iter() {
                device.destroy_image_view(swapchain_image_view, None);
            }

            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }

    pub fn create(
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &vk::SurfaceKHR,
        suitable_device: &SuitableDevice,
        device: &ash::Device,
        queue: &vk::Queue,
        transient_command_pool: &vk::CommandPool,
        descriptor_set_layout: vk::DescriptorSetLayout,
        window_size: (u32, u32),
    ) -> RisResult<Self> {
        let SurfaceDetails{
            capabilities,
            formats,
            present_modes,
        } = SurfaceDetails::query(
            &surface_loader,
            suitable_device.physical_device,
            *surface,
        )?;

        // swap chain
        let preferred_surface_format = formats
            .iter()
            .find(|x| x.format == super::PREFERRED_FORMAT && x.color_space == super::PREFERRED_COLOR_SPACE);
        let surface_format = match preferred_surface_format {
            Some(format) => format,
            // getting the first format if the preferred format does not exist. this should not
            // cause ub, becuase we checked if the list is empty at finding the suitable device.
            None => &formats[0],
        };

        let preferred_surface_present_mode = present_modes
            .iter()
            .find(|&&x| x == super::PREFERRED_PRESENT_MODE);
        let surface_present_mode = match preferred_surface_present_mode {
            Some(present_mode) => present_mode,
            // getting the first present mode if the preferred format does not exist. this should
            // not cause ub, because we checked if the list is empty at finding the suitable device.
            None => unsafe{present_modes.get_unchecked(0)},
        };

        let swapchain_extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let (window_width, window_height) = window_size;
            let width = window_width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            let height = window_height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            vk::Extent2D {
                width,
                height,
            }
        };

        let preferred_swapchain_image_count = capabilities.min_image_count + 1;
        let swapchain_image_count = if capabilities.max_image_count == 0 {
            // SurfaceCapabilitiesKHR == 0 indicates there is no maximum
            preferred_swapchain_image_count
        } else {
            u32::min(
                preferred_swapchain_image_count,
                capabilities.max_image_count,
            )
        };

        let (image_sharing_mode, queue_family_index_count, queue_family_indices) = 
            if suitable_device.graphics_queue_family != suitable_device.present_queue_family {(
                vk::SharingMode::CONCURRENT,
                2,
                vec![
                suitable_device.graphics_queue_family,
                suitable_device.present_queue_family,
                ],
            )} else {(
                vk::SharingMode::EXCLUSIVE,
                0,
                vec![],
            )};

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: *surface,
            min_image_count: swapchain_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: swapchain_extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count,
            pre_transform: capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: *surface_present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            image_array_layers: 1,
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_create_info, None)
        }?;

        let swapchain_images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)
        }?;

        // image views
        let mut swapchain_image_views = Vec::new();
        for swapchain_image in swapchain_images.iter() {
            let image_view = Image::alloc_view(
                device,
                *swapchain_image,
                surface_format.format,
                vk::ImageAspectFlags::COLOR,
            )?;

            swapchain_image_views.push(image_view);
        }

        // graphics pipeline
        let graphics_pipeline = GraphicsPipeline::alloc(
            instance,
            suitable_device.physical_device,
            device,
            surface_format.format,
            swapchain_extent,
            descriptor_set_layout,
        )?;

        // depth buffer
        let depth_format = util::find_depth_format(
            instance,
            suitable_device.physical_device,
        )?;

        let physical_device_memory_properties = unsafe{
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let depth_image = Image::alloc(
            device,
            swapchain_extent.width,
            swapchain_extent.height,
            depth_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        let depth_image_view = Image::alloc_view(
            device,
            depth_image.image,
            depth_format,
            vk::ImageAspectFlags::DEPTH,
        )?;

        depth_image.transition_layout(
            device,
            queue,
            transient_command_pool,
            depth_format,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        )?;

        // frame buffers
        let mut framebuffers = Vec::with_capacity(swapchain_image_views.len());
        for &swapchain_image_view in swapchain_image_views.iter() {
            let attachments = [
                swapchain_image_view,
                depth_image_view,
            ];

            let framebuffer_create_info = vk::FramebufferCreateInfo {
                s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::FramebufferCreateFlags::empty(),
                render_pass: graphics_pipeline.render_pass, 
                attachment_count: attachments.len() as u32,
                p_attachments: attachments.as_ptr(),
                width: swapchain_extent.width,
                height: swapchain_extent.height,
                layers: 1,
            };

            let framebuffer = unsafe{device.create_framebuffer(&framebuffer_create_info, None)}?;
            framebuffers.push(framebuffer);
        }

        Ok(Self{
            swapchain_loader,
            swapchain,
            swapchain_format: *surface_format,
            swapchain_extent,
            swapchain_images,
            swapchain_image_views,
            graphics_pipeline,
            depth_image,
            depth_image_view,
            framebuffers,
        })
    }
}
