#![cfg_attr(feature = "ris_windows_subsystem", windows_subsystem = "windows")]

fn main() -> ris_error::RisResult<()> {
    let ecs = ris_ecs::ecs::Ecs::new(42);

    let info = ris_data::package_info!();
    ris_core::entry::run(info)
}
