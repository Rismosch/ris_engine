use ash::extensions::khr::Surface as SurfaceLoader;
use ash::extensions::khr::Swapchain as SwapchainLoader;
use ash::vk;

use ris_error::prelude::*;
use ris_ptr::ArefCell;

use super::frames_in_flight::RendererId;
use super::image::Image;
use super::image::ImageCreateInfo;
use super::image::TransitionLayoutInfo;
use super::suitable_device::SuitableDevice;
use super::surface_details::SurfaceDetails;
use super::transient_command::TransientCommandArgs;

pub struct Swapchain {
    pub format: vk::SurfaceFormatKHR,
    pub depth_format: vk::Format,
    pub extent: vk::Extent2D,
    pub loader: SwapchainLoader,
    pub swapchain: vk::SwapchainKHR,
    pub entries: Vec<SwapchainEntry>,
}

type Framebuffer = ArefCell<Option<vk::Framebuffer>>;

pub struct SwapchainEntry {
    pub index: usize,
    pub viewport_image: vk::Image,
    pub viewport_image_view: vk::ImageView,
    pub depth_image: Image,
    pub depth_image_view: vk::ImageView,
    framebuffers: Vec<Framebuffer>,
}

pub struct SwapchainCreateInfo<'a> {
    pub instance: &'a ash::Instance,
    pub surface_loader: &'a SurfaceLoader,
    pub surface: &'a vk::SurfaceKHR,
    pub suitable_device: &'a SuitableDevice,
    pub device: &'a ash::Device,
    pub graphics_queue: vk::Queue,
    pub transient_command_pool: vk::CommandPool,
    pub window_drawable_size: (u32, u32),
}

impl Swapchain {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            for framebuffer in entry.framebuffers.iter_mut() {
                if let Some(framebuffer) = framebuffer.borrow_mut().take() {
                    device.destroy_framebuffer(framebuffer, None);
                }
            }

            device.destroy_image_view(entry.viewport_image_view, None);
            entry.depth_image.free(device);
            device.destroy_image_view(entry.depth_image_view, None);
        }

        self.loader.destroy_swapchain(self.swapchain, None);
    }

    pub fn alloc(info: SwapchainCreateInfo) -> RisResult<Self> {
        let SwapchainCreateInfo {
            instance,
            surface_loader,
            surface,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
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

        let depth_format = super::util::find_supported_format(
            instance,
            suitable_device.physical_device,
            &[
                vk::Format::D24_UNORM_S8_UINT,
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        )?;

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

        // sticking with min_image_count may result in us waiting
        // on the driver. to prevent this, we prefer one more
        // image on the swapchain. of course this may not exceed
        // max_image_count
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
            p_next: std::ptr::null(),
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

        // create entries
        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let viewport_images = unsafe { loader.get_swapchain_images(swapchain) }?;

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::FenceCreateFlags::empty(),
        };

        let mut fences = Vec::with_capacity(viewport_images.len());
        let mut entries = Vec::with_capacity(viewport_images.len());
        for (index, viewport_image) in viewport_images.into_iter().enumerate() {
            let viewport_image_view = Image::alloc_view(
                device.clone(),
                viewport_image,
                format.format,
                vk::ImageAspectFlags::COLOR,
            )?;

            let mut depth_image = Image::alloc(ImageCreateInfo {
                device: device.clone(),
                width: extent.width as usize,
                height: extent.height as usize,
                format: depth_format,
                usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                physical_device_memory_properties,
            })?;

            let depth_image_view = Image::alloc_view(
                device.clone(),
                depth_image.image,
                depth_format,
                vk::ImageAspectFlags::DEPTH,
            )?;

            let fence = unsafe { device.create_fence(&fence_create_info, None) }?;
            fences.push(fence);

            depth_image.transition_layout(TransitionLayoutInfo {
                transient_command_args: TransientCommandArgs {
                    device: device.clone(),
                    queue: graphics_queue,
                    command_pool: transient_command_pool,
                },
                new_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                fence: Some(fence),
            })?;

            let entry = SwapchainEntry {
                index,
                viewport_image,
                viewport_image_view,
                depth_image,
                depth_image_view,
                framebuffers: Vec::new(),
            };
            entries.push(entry);
        }

        unsafe {
            device.wait_for_fences(&fences, true, u64::MAX)?;
            for fence in fences {
                device.destroy_fence(fence, None);
            }
        }

        Ok(Self {
            format,
            depth_format,
            extent,
            loader,
            swapchain,
            entries,
        })
    }

    pub fn reserve_framebuffers(&mut self, image_index: usize, count: usize) {
        let entry = &mut self.entries[image_index];
        while entry.framebuffers.len() < count {
            entry.framebuffers.push(ArefCell::new(None));
        }
    }
}

impl SwapchainEntry {
    pub fn alloc_framebuffer(
        &self,
        id: RendererId,
        device: &ash::Device,
        framebuffer_create_info: vk::FramebufferCreateInfo,
    ) -> RisResult<vk::Framebuffer> {
        let index = id.index();
        let mut framebuffer = self.framebuffers.get(index).into_ris_error()?.borrow_mut();

        match *framebuffer {
            Some(framebuffer) => Ok(framebuffer),
            None => {
                let new_framebuffer =
                    unsafe { device.create_framebuffer(&framebuffer_create_info, None) }?;
                *framebuffer = Some(new_framebuffer);
                Ok(new_framebuffer)
            }
        }
    }
}
