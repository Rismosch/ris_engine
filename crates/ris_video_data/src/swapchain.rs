use std::ptr;

use ash::extensions::khr::Surface as SurfaceLoader;
use ash::extensions::khr::Swapchain as SwapchainLoader;
use ash::vk;

use ris_error::prelude::*;
use ris_ptr::ArefCell;

use super::image::Image;
use super::image::ImageCreateInfo;
use super::image::TransitionLayoutInfo;
use super::suitable_device::SuitableDevice;
use super::surface_details::SurfaceDetails;
use super::transient_command::TransientCommandSync;

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RendererId {
    index: usize,
    command_buffers_start: usize,
    command_buffers_end: usize,
}

pub struct FrameInFlight {
    pub image_available: vk::Semaphore,
    pub renderer_finished: Vec<vk::Semaphore>,
    pub command_buffer_finished: vk::Semaphore,
    pub done: vk::Fence,
}

pub struct Swapchain {
    pub format: vk::SurfaceFormatKHR,
    pub extent: vk::Extent2D,
    pub loader: SwapchainLoader,
    pub swapchain: vk::SwapchainKHR,
    pub entries: Vec<SwapchainEntry>,
    renderer_count: usize,
    command_buffer_count: usize,
}

pub struct SwapchainEntry {
    pub index: usize,
    pub viewport_image: vk::Image,
    pub viewport_image_view: vk::ImageView,
    pub depth_format: vk::Format,
    pub depth_image: Image,
    pub depth_image_view: vk::ImageView,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>, // multiple per renderer
    pub framebuffers: Vec<Option<vk::Framebuffer>>, // one per renderer
    pub frame_in_flight: FrameInFlight,
}

pub struct SwapchainCreateInfo<'a> {
    pub instance: &'a ash::Instance,
    pub suitable_device: &'a SuitableDevice,
    pub device: &'a ash::Device,
    pub surface_loader: &'a SurfaceLoader,
    pub surface: &'a vk::SurfaceKHR,
    pub window_drawable_size: (u32, u32),
}

pub struct SwapchainEntryCreateInfo<'a> {
    pub instance: &'a ash::Instance,
    pub suitable_device: &'a SuitableDevice,
    pub device: &'a ash::Device,
    pub graphics_queue: vk::Queue,
    pub transient_command_pool: vk::CommandPool,
}

impl Swapchain {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        unsafe {
            for entry in self.entries.iter_mut() {
                // free frame in flight
                device.destroy_fence(
                    entry.frame_in_flight.execution_finished,
                    None,
                );
                for &semaphore in entry.frame_in_flight.renderer_finished.iter() {
                    device.destroy_semaphore(
                        semaphore,
                        None,
                    );
                }
                device.destroy_semaphore(
                    entry.frame_in_flight.image_available,
                    None,
                );

                // free frame buffers
                for framebuffer in entry.framebuffers.iter_mut() {
                    if let Some(framebuffer) = framebuffer.take() {
                        device.destroy_framebuffer(framebuffer, None);
                    }
                }

                // free command buffers
                device.free_command_buffers(entry.command_pool, &entry.command_buffers);
                device.destroy_command_pool(entry.command_pool, None);

                // free images
                device.destroy_image_view(entry.viewport_image_view, None);
                entry.depth_image.free(device);
                device.destroy_image_view(entry.depth_image_view, None);
            }

            self.loader.destroy_swapchain(self.swapchain, None);
        }
    }

    pub fn alloc(info: SwapchainCreateInfo) -> RisResult<Self> {
        ris_error::new_result!("initialization order:
            1. swapchain
            2. renderers
            3. swapchain entries
        ")?;

        let SwapchainCreateInfo {
            instance,
            suitable_device,
            device,
            surface_loader,
            surface,
            window_drawable_size,
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

        Ok(Self {
            format,
            extent,
            loader,
            swapchain,
            entries: Vec::new(),
            renderer_count: 0,
            command_buffer_count: 0,
        })
    }

    pub fn register_renderer(&mut self, command_buffer_count: usize) -> RendererId {
        let index = self.renderer_count;
        let command_buffers_start = self.command_buffer_count;
        let command_buffers_end = command_buffers_start + command_buffer_count;

        self.renderer_count += 1;
        self.command_buffer_count += command_buffer_count;

        RendererId {
            index,
            command_buffers_start,
            command_buffers_end,
        }
    }

    pub fn alloc_entries(&mut self, info: SwapchainEntryCreateInfo) -> RisResult<()> {
        if !self.entries.is_empty() {
            return ris_error::new_result!("swapchain entries are already allocated");
        }

        let Self {
            format,
            extent,
            loader,
            swapchain,
            entries,
            renderer_count,
            command_buffer_count,
        } = self;

        let SwapchainEntryCreateInfo {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
        } = info;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let depth_format =
            super::util::find_depth_format(instance, suitable_device.physical_device)?;

        let viewport_images = unsafe { loader.get_swapchain_images(*swapchain) }?;

        for (i, viewport_image) in viewport_images.into_iter().enumerate() {
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

            // command buffers
            ris_error::new_result!("todo")?;

            // frame buffers
            let framebuffers = vec![None; *renderer_count];

            // frame in flight
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

            let image_available = unsafe {device.create_semaphore(&semaphore_create_info, None)}?;

            let mut renderer_finished = Vec::with_capacity(*renderer_count);
            for _ in 0..renderer_finished.capacity() {
                let semaphore = unsafe {device.create_semaphore(&semaphore_create_info, None)}?;
                renderer_finished.push(semaphore);
            }

            let command_buffer_finished = unsafe {device.create_semaphore(&semaphore_create_info, None)}?;

            let done = unsafe {device.create_fence(&fence_create_info, None)}?;

            let frame_in_flight = FrameInFlight {
                image_available,
                renderer_finished,
                command_buffer_finished,
                done,
            };

            // construct entry
            let entry = SwapchainEntry {
                index: i,
                viewport_image,
                viewport_image_view,
                depth_format,
                depth_image,
                depth_image_view,
                command_pool,
                command_buffers,
                framebuffers,
                frame_in_flight,
            };
            self.entries.push(entry);
        }

        Ok(())
    }
}

impl SwapchainEntry {
    pub fn command_buffer(&self, id: &RendererId) -> RisError<&[vk::CommandBuffer]> {
        let RendererId {
            command_buffer_range,
            ..
        } = id;

        self.command_buffers.get(id.0).into_ris_error()
    }

    pub fn frame_buffer(
        &mut self,
        id: &RendererId,
        device: &ash::Device,
        framebuffer_create_info: vk::FramebufferCreateInfo,
    ) -> RisResult<vk::Framebuffer> {
        let index = id.index;

        let framebuffer = self.framebuffers.get_mut(index).into_ris_error()?;

        match framebuffer {
            Some(framebuffer) => Ok(*framebuffer),
            None => {
                let new_framebuffer = device.create_framebuffer(&framebuffer_create_info, None)?;
                *framebuffer = Some(new_framebuffer);
                Ok(new_framebuffer)
            }
        }
    }

    pub fn semaphore(&self, id: &RendererId) -> vk::Semaphore {
        let index = id.index as isize;
        self.semaphore_internal(index)
    }

    pub fn previous_semaphore(&self, id: &RendererId) -> vk::Semaphore {
        let index = id.index as isize - 1;
        self.semaphore_internal(index)
    }

    fn semaphore_internal(&self, index: isize) -> vk::Semaphore {
        if index < 0 {
            self.frame_in_flight.image_available
        } else {
            self.frame_in_flight.renderer_finished[index as usize]
        }
    }
}
