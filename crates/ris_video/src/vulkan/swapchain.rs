use std::ptr;

use ash::extensions::khr::Surface as SurfaceLoader;
use ash::extensions::khr::Swapchain as SwapchainLoader;
use ash::vk;

use ris_error::RisResult;

use super::frame_in_flight::FrameInFlight;
use super::image::Image;
use super::image::ImageCreateInfo;
use super::image::TransitionLayoutInfo;
use super::suitable_device::SuitableDevice;
use super::surface_details::SurfaceDetails;
use super::transient_command::TransientCommandSync;

pub struct Swapchain {
    pub format: vk::SurfaceFormatKHR,
    pub extent: vk::Extent2D,
    pub loader: SwapchainLoader,
    pub swapchain: vk::SwapchainKHR,
    pub entries: Vec<SwapchainEntry>,
    pub frames_in_flight: Option<Vec<FrameInFlight>>,
    command_buffers: Vec<vk::CommandBuffer>,
}

pub struct SwapchainEntry {
    pub index: usize,
    pub viewport_image: vk::Image,
    pub viewport_image_view: vk::ImageView,
    pub depth_image: Image,
    pub depth_image_view: vk::ImageView,
    pub command_buffer: vk::CommandBuffer,
}

pub struct SwapchainCreateInfo<'a> {
    pub instance: &'a ash::Instance,
    pub suitable_device: &'a SuitableDevice,
    pub device: &'a ash::Device,
    pub graphics_queue: vk::Queue,
    pub command_pool: vk::CommandPool,
    pub transient_command_pool: vk::CommandPool,
    pub surface_loader: &'a SurfaceLoader,
    pub surface: &'a vk::SurfaceKHR,
    pub window_drawable_size: (u32, u32),
    pub frames_in_flight: Option<Vec<FrameInFlight>>,
}

impl Swapchain {
    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device, command_pool: vk::CommandPool) {
        unsafe {
            device.free_command_buffers(command_pool, &self.command_buffers);

            if let Some(frames_in_flight) = self.frames_in_flight.take() {
                for frame_in_flight in frames_in_flight.iter() {
                    frame_in_flight.free(device);
                }
            }

            for entry in self.entries.iter_mut() {
                device.destroy_image_view(entry.viewport_image_view, None);
                entry.depth_image.free(device);
                device.destroy_image_view(entry.depth_image_view, None);
            }

            self.loader.destroy_swapchain(self.swapchain, None);
        }
    }

    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(info: SwapchainCreateInfo) -> RisResult<Self> {
        let SwapchainCreateInfo {
            instance,
            suitable_device,
            device,
            graphics_queue,
            command_pool,
            transient_command_pool,
            surface_loader,
            surface,
            window_drawable_size,
            frames_in_flight,
        } = info;

        let SurfaceDetails {
            capabilities,
            formats,
            present_modes,
        } = SurfaceDetails::query(surface_loader, suitable_device.physical_device, *surface)?;

        let preferred_surface_format = formats.iter().find(|x| {
            x.format == super::PREFERRED_FORMAT && x.color_space == super::PREFERRED_COLOR_SPACE
        });
        let format = match preferred_surface_format {
            Some(format) => *format,
            None => formats[0],
        };

        let preferred_surface_present_mode = present_modes
            .iter()
            .find(|&&x| x == super::PREFERRED_PRESENT_MODE);
        let surface_present_mode = match preferred_surface_present_mode {
            Some(present_mode) => present_mode,
            None => &present_modes[0],
        };

        let extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let (window_width, window_height) = window_drawable_size;
            let width = window_width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            let height = window_height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            vk::Extent2D { width, height }
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

        let (image_sharing_mode, queue_family_indices) =
            if suitable_device.graphics_queue_family == suitable_device.present_queue_family {
                (vk::SharingMode::EXCLUSIVE, vec![])
            } else {
                (
                    vk::SharingMode::CONCURRENT,
                    vec![
                        suitable_device.graphics_queue_family,
                        suitable_device.present_queue_family,
                    ],
                )
            };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: *surface,
            min_image_count: swapchain_image_count,
            image_format: format.format,
            image_color_space: format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            queue_family_index_count: queue_family_indices.len() as u32,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            pre_transform: capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: *surface_present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
        };

        // create swapchain
        let loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swapchain = unsafe { loader.create_swapchain(&swapchain_create_info, None) }?;
        let images = unsafe { loader.get_swapchain_images(swapchain) }?;

        // command buffers
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: images.len() as u32,
        };

        let command_buffers =
            unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }?;

        // swapchain entries
        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let depth_format =
            super::util::find_depth_format(instance, suitable_device.physical_device)?;

        let mut entries = Vec::with_capacity(images.len());
        for (i, viewport_image) in images.into_iter().enumerate() {
            // viewport view
            let viewport_image_view = Image::alloc_view(
                device,
                viewport_image,
                format.format,
                vk::ImageAspectFlags::COLOR,
            )?;

            // depth
            let depth_image = Image::alloc(ImageCreateInfo {
                device,
                width: extent.width,
                height: extent.height,
                format: depth_format,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                memory_property_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                physical_device_memory_properties,
            })?;

            let depth_image_view = Image::alloc_view(
                device,
                depth_image.image,
                depth_format,
                vk::ImageAspectFlags::DEPTH,
            )?;

            depth_image.transition_layout(TransitionLayoutInfo {
                device,
                queue: graphics_queue,
                transient_command_pool,
                format: depth_format,
                old_layout: vk::ImageLayout::UNDEFINED,
                new_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                sync: TransientCommandSync::default(),
            })?;

            // command buffer
            let command_buffer = command_buffers[i];

            // entry
            let swapchain_entry = SwapchainEntry {
                index: i,
                viewport_image,
                viewport_image_view,
                depth_image,
                depth_image_view,
                command_buffer,
            };

            entries.push(swapchain_entry);
        } // end swapchain entries

        // frames in flight
        let frames_in_flight = match frames_in_flight {
            Some(x) => Some(x),
            None => {
                let mut frames_in_flight = Vec::with_capacity(command_buffers.len());
                for _ in 0..super::MAX_FRAMES_IN_FLIGHT {
                    let frame_in_flight = FrameInFlight::alloc(device)?;

                    frames_in_flight.push(frame_in_flight);
                }

                Some(frames_in_flight)
            }
        };

        Ok(Self {
            format,
            extent,
            loader,
            swapchain,
            entries,
            frames_in_flight,
            command_buffers,
        })
    }
}
