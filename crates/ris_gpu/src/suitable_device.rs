use std::ffi::CStr;

use ash::vk;

use ris_error::RisResult;

use super::surface_details::SurfaceDetails;

const LIST_ALL_AVAILABLE_EXTENSIONS: bool = false;

type Extension = &'static CStr;
struct PreferredExtension {
    name: Extension,
    dependencies: &'static [Extension],
}

const VK_KHR_SWAPCHAIN: Extension = ash::extensions::khr::Swapchain::name();
const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2: Extension =
    ash::extensions::khr::GetPhysicalDeviceProperties2::name();
const VK_EXT_MEMORY_PRIORITY: Extension = ash::vk::ExtMemoryPriorityFn::name();
const VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY: Extension =
    ash::vk::ExtPageableDeviceLocalMemoryFn::name();

const REQUIRED_DEVICE_EXTENSIONS: &[Extension] = &[VK_KHR_SWAPCHAIN];

const PREFERRED_DEVICE_EXTENSIONS: &[PreferredExtension] = &[PreferredExtension {
    name: VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY,
    dependencies: &[
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2,
        VK_EXT_MEMORY_PRIORITY,
    ],
}];

pub struct SuitableDevice {
    pub name: String,
    // the lower the suitability, the better suited the device is to render. a dedicated gpu would
    // have a value of 0
    pub suitability: usize,
    pub graphics_queue_family: u32,
    pub present_queue_family: u32,
    pub physical_device: vk::PhysicalDevice,
    pub extensions: Vec<Extension>,
}

impl SuitableDevice {
    pub fn query(
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: vk::SurfaceKHR,
    ) -> RisResult<Vec<Self>> {
        let physical_devices = unsafe { instance.enumerate_physical_devices()? };

        let mut suitable_devices = Vec::new();

        let mut log_message = "Vulkan Device Extensions:".to_string();
        log_message.push_str(&format!(
            "\n\trequired: {}",
            REQUIRED_DEVICE_EXTENSIONS.len()
        ));
        for &extension in REQUIRED_DEVICE_EXTENSIONS {
            let extension_str = extension.to_str()?;
            log_message.push_str(&format!("\n\t- {}", extension_str));
        }
        log_message.push_str(&format!(
            "\n\tpreferred: {}",
            PREFERRED_DEVICE_EXTENSIONS.len()
        ));
        for extension in PREFERRED_DEVICE_EXTENSIONS {
            let extension_str = extension.name.to_str()?;
            log_message.push_str(&format!("\n\t- {}", extension_str));
        }
        ris_log::debug!("{}", log_message);

        // find suitable physical devices
        for (i, &physical_device) in physical_devices.iter().enumerate() {
            // gather physical device information
            let device_properties =
                unsafe { instance.get_physical_device_properties(physical_device) };
            let device_features = unsafe { instance.get_physical_device_features(physical_device) };
            let device_queue_families =
                unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

            let (suitability, device_type_name) = match device_properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => (0, "discrete gpu"),
                vk::PhysicalDeviceType::INTEGRATED_GPU => (1, "integrated gpu"),
                vk::PhysicalDeviceType::VIRTUAL_GPU => (2, "virtual gpu"),
                vk::PhysicalDeviceType::CPU => (3, "cpu"),
                vk::PhysicalDeviceType::OTHER => (4, "unkown"),
                _ => continue,
            };

            let mut log_message = format!("Vulkan Physical Device {}", i);

            let device_name = super::util::vk_to_std_str(&device_properties.device_name)?;
            log_message.push_str(&format!("\n\tname: {}", device_name));
            log_message.push_str(&format!("\n\tid: {}", device_properties.device_id));
            log_message.push_str(&format!("\n\ttype: {}", device_type_name));

            let api_version_variant = vk::api_version_variant(device_properties.api_version);
            let api_version_major = vk::api_version_major(device_properties.api_version);
            let api_version_minor = vk::api_version_minor(device_properties.api_version);
            let api_version_patch = vk::api_version_patch(device_properties.api_version);
            let api_version = format!(
                "{}.{}.{}.{}",
                api_version_variant, api_version_major, api_version_minor, api_version_patch,
            );
            log_message.push_str(&format!("\n\tapi version: {}", api_version));

            log_message.push_str(&format!(
                "\n\tsupported queue families: {}",
                device_queue_families.len()
            ));
            log_message.push_str("\n\t\tqueue | graphics, compute, transfer, sparse binding");
            for queue_family in device_queue_families.iter() {
                let supports_graphics = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                let supports_compute = queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE);
                let supports_transfer = queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER);
                let supports_sparse_binding = queue_family
                    .queue_flags
                    .contains(vk::QueueFlags::SPARSE_BINDING);

                log_message.push_str(&format!(
                    "\n\t\t{:5} | {:8}, {:7}, {:8}, {:14}",
                    queue_family.queue_count,
                    supports_graphics,
                    supports_compute,
                    supports_transfer,
                    supports_sparse_binding,
                ));
            }

            log_message.push_str(&format!(
                "\n\tgeometry shader support: {}",
                device_features.geometry_shader == vk::TRUE
            ));

            // check device extension support
            let available_extensions =
                unsafe { instance.enumerate_device_extension_properties(physical_device)? };

            let mut supports_required_extensions = true;
            let mut extensions = Vec::new();

            for &required_extension in REQUIRED_DEVICE_EXTENSIONS {
                if !extension_exists(required_extension, &available_extensions)? {
                    supports_required_extensions = false;
                    break;
                }

                extensions.push(required_extension);
            }

            for preferred_extension in PREFERRED_DEVICE_EXTENSIONS {
                if !extension_exists(preferred_extension.name, &available_extensions)? {
                    continue;
                }

                let mut can_enable_extension = true;
                let mut dependencies_to_enable = Vec::new();
                for &dependency in preferred_extension.dependencies {
                    let mut dependency_is_already_enabled = false;
                    for &extension in extensions.iter() {
                        if extension == dependency {
                            dependency_is_already_enabled = true;
                            break;
                        }
                    }

                    if dependency_is_already_enabled {
                        continue;
                    }

                    if !extension_exists(dependency, &available_extensions)? {
                        can_enable_extension = false;
                        break;
                    }

                    dependencies_to_enable.push(dependency);
                }

                if can_enable_extension {
                    for dependency in dependencies_to_enable {
                        extensions.push(dependency);
                    }

                    extensions.push(preferred_extension.name);
                }
            }

            log_message.push_str(&format!(
                "\n\trequired extension support: {}",
                supports_required_extensions
            ));
            log_message.push_str(&format!(
                "\n\textensions to be enabled: {}",
                extensions.len(),
            ));
            for &extension in extensions.iter() {
                let name = extension.to_str()?;
                log_message.push_str(&format!("\n\t\t- {}", name));
            }
            if LIST_ALL_AVAILABLE_EXTENSIONS {
                log_message.push_str(&format!(
                    "\n\tavailable extensions: {}",
                    available_extensions.len()
                ));
                for extension in available_extensions {
                    let name = super::util::vk_to_std_str(&extension.extension_name)?;
                    log_message.push_str(&format!("\n\t\t- {}", name));
                }
            }

            // check swapchain support
            let SurfaceDetails {
                formats,
                present_modes,
                ..
            } = SurfaceDetails::query(surface_loader, physical_device, surface)?;

            log_message.push_str(&format!("\n\tsurface formats: {}", formats.len()));
            for format in formats.iter() {
                log_message.push_str(&format!(
                    "\n\t\t- {:?}, {:?}",
                    format.format, format.color_space
                ));
            }
            log_message.push_str(&format!(
                "\n\tsurface present modes: {}",
                present_modes.len()
            ));
            for present_mode in present_modes.iter() {
                log_message.push_str(&format!("\n\t\t- {:?}", present_mode));
            }

            ris_log::info!("{}", log_message);

            if !supports_required_extensions || formats.is_empty() || present_modes.is_empty() {
                continue; // device not supported. skip
            }

            // find queue family
            let mut queue_supports = Vec::with_capacity(device_queue_families.len());

            for (i, queue_family) in device_queue_families.iter().enumerate() {
                if queue_family.queue_count == 0 {
                    continue;
                }

                let graphics_queue_index = queue_family
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS)
                    .then_some(i);
                let present_queue_index = unsafe {
                    surface_loader.get_physical_device_surface_support(
                        physical_device,
                        i as u32,
                        surface,
                    )
                }?
                .then_some(i);

                queue_supports.push((i, graphics_queue_index, present_queue_index));
            }

            // a preferred queue supports all flags
            let preferred_queue = queue_supports
                .iter()
                .find(|x| x.1.is_some() && x.2.is_some());

            let (graphics, present) = match preferred_queue {
                Some(&(i, ..)) => (i, i),
                None => {
                    // no single queue exists, which supports all flags. attempt to find multiple
                    // queues that together support all flags
                    let mut graphics = None;
                    let mut present = None;

                    for (i, graphics_queue_index, present_queue_index) in queue_supports {
                        if graphics_queue_index.is_some() {
                            graphics = Some(i);
                        }

                        if present_queue_index.is_some() {
                            present = Some(i);
                        }
                    }

                    if let (Some(graphics), Some(present)) = (graphics, present) {
                        (graphics, present)
                    } else {
                        continue; // device does not have all necessary queues. skip
                    }
                }
            };

            let suitable_device = SuitableDevice {
                name: device_name.to_string(),
                suitability,
                graphics_queue_family: graphics as u32,
                present_queue_family: present as u32,
                physical_device,
                extensions,
            };
            suitable_devices.push(suitable_device);
        } // end find suitable physical devices

        Ok(suitable_devices)
    }
}

fn extension_exists(
    extension: &CStr,
    available_extensions: &[vk::ExtensionProperties],
) -> RisResult<bool> {
    for available_extension in available_extensions.iter() {
        let left = super::util::vk_to_std_str(&available_extension.extension_name)?;
        let right = extension.to_str()?;
        if left == right {
            return Ok(true);
        }
    }
    Ok(false)
}
