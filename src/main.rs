#![cfg_attr(feature = "ris_windows_subsystem", windows_subsystem = "windows")]

fn main() -> ris_error::RisResult<()> {
    todo!("remove compression");
    let info = ris_data::package_info!();
    ris_core::entry::run(info)
}
