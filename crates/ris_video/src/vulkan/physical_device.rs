use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::DeviceExtensions;
use vulkano::device::QueueFlags;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use ris_error::Extensions;
use ris_error::RisResult;

pub fn select_physical_device(
    instance: Arc<Instance>,
    surface: Arc<Surface>,
    device_extensions: &DeviceExtensions,
) -> RisResult<(Arc<PhysicalDevice>, u32)> {
    let available_devices = instance
        .enumerate_physical_devices()?
        .filter(|p| p.supported_extensions().contains(device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|q| (p, q as u32))
        })
        .collect::<Vec<_>>();

    let log_string = match available_devices.len() {
        0 => String::from("no available devices"),
        len => {
            let s = if len > 1 { "s" } else { "" };

            let mut log_string = format!("{} available video device{}:", len, s);
            for (device, i) in available_devices.iter() {
                let properties = device.properties();

                log_string.push_str(&format!(
                    "\n    [{}] => {:?}: {}",
                    i, properties.device_type, properties.device_name,
                ));
            }

            log_string
        }
    };

    ris_log::info!("{}", log_string);

    let result = available_devices
        .into_iter()
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .unroll()?;

    let properties = result.0.properties();

    ris_log::info!(
        "selected physical video device {:?}: {}",
        properties.device_type,
        properties.device_name,
    );

    Ok(result)
}
