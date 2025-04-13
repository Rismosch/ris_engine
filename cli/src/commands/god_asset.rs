use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_asset::assets::ris_god_asset;
use ris_asset::assets::ris_god_asset::RisGodAsset;
use ris_data::asset_id::AssetId;
use ris_data::info::args_info::DEFAULT_ASSETS_VALUE;
use ris_error::RisResult;

use crate::ExplanationLevel;
use crate::ICommand;

const FLAG_INPUT: &str = "-i";
const CMD_PRINT: &str = "print";
const CMD_SET: &str = "set";

pub struct GodAsset;

fn default_asset_path() -> PathBuf {
    PathBuf::from(DEFAULT_ASSETS_VALUE).join(ris_god_asset::PATH)
}

impl ICommand for GodAsset {
    fn name(&self) -> String {
        "god_asset".to_string()
    }

    fn args(&self) -> String {
        "[-i <filepath>] <command> [args...]".to_string()
    }

    fn explanation(&self, level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => {
                String::from("prints or modifies the god asset, the entry point of all assets.")
            }
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();
                let short = self.explanation(ExplanationLevel::Short);
                explanation.push_str(&format!("{}\n", short));
                explanation.push('\n');
                explanation.push_str("flags:\n");
                explanation.push('\n');
                explanation.push_str(&format!("[{} <filepath>]\n", FLAG_INPUT));
                explanation.push_str("the path to the god asset.\n");
                explanation.push_str(&format!(
                    "default: \"{}\"\n",
                    default_asset_path().display(),
                ));
                explanation.push('\n');
                explanation.push_str("commands:\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", CMD_PRINT));
                explanation.push_str("prints the current god asset.\n");
                explanation.push('\n');
                explanation.push_str(&format!("{} <field> <value>\n", CMD_SET));
                explanation.push_str("sets the <field> of the god asset with <value> and writes the changed asset.\n");

                explanation
            }
        }
    }

    fn run(&self, args: Vec<String>, _target_dir: PathBuf) -> RisResult<()> {
        let flag_input = args.iter().position(|x| x == FLAG_INPUT);
        let (command_index, god_asset_path) = match flag_input {
            Some(flag_input_position) => {
                let command_index = flag_input_position + 2;
                let flag_input_arg = args
                    .get(flag_input_position + 1)
                    .ok_or(ris_error::new!("too few arguments"))?;
                let god_asset_path = PathBuf::from(flag_input_arg);

                (command_index, god_asset_path)
            }
            None => (2, default_asset_path()),
        };

        let command = args
            .get(command_index)
            .ok_or(ris_error::new!("too few arguments"))?;

        match command.as_str() {
            CMD_PRINT => {
                let god_asset = read_god_asset(god_asset_path)?;
                print_god_asset(&god_asset);
                Ok(())
            }
            CMD_SET => {
                let field = args
                    .get(command_index + 1)
                    .ok_or(ris_error::new!("too few arguments"))?;
                let value = args
                    .get(command_index + 2)
                    .ok_or(ris_error::new!("too few arguments"))?
                    .clone();

                let mut god_asset = read_god_asset(&god_asset_path)?;

                match field.trim().to_lowercase().as_str() {
                    "default_vert_spv" => god_asset.default_vert_spv = AssetId::Path(value),
                    "default_frag_spv" => god_asset.default_frag_spv = AssetId::Path(value),
                    "imgui_vert_spv" => god_asset.imgui_vert_spv = AssetId::Path(value),
                    "imgui_frag_spv" => god_asset.imgui_frag_spv = AssetId::Path(value),
                    "gizmo_segment_vert_spv" => {
                        god_asset.gizmo_segment_vert_spv = AssetId::Path(value)
                    }
                    "gizmo_segment_frag_spv" => {
                        god_asset.gizmo_segment_frag_spv = AssetId::Path(value)
                    }
                    "gizmo_text_vert_spv" => god_asset.gizmo_text_vert_spv = AssetId::Path(value),
                    "gizmo_text_geom_spv" => god_asset.gizmo_text_geom_spv = AssetId::Path(value),
                    "gizmo_text_frag_spv" => god_asset.gizmo_text_frag_spv = AssetId::Path(value),
                    "debug_font_texture" => god_asset.debug_font_texture = AssetId::Path(value),
                    "texture" => god_asset.texture = AssetId::Path(value),
                    _ => return ris_error::new_result!("unkown field \"{}\"", field),
                }

                write_god_asset(&god_asset, god_asset_path)?;
                print_god_asset(&god_asset);

                Ok(())
            }
            _ => ris_error::new_result!("unkown command: {}", command),
        }
    }
}

fn read_god_asset(path: impl AsRef<Path>) -> RisResult<RisGodAsset> {
    let path = path.as_ref();
    eprintln!("reading god_asset... \"{}\"", path.display());
    let mut file = std::fs::File::open(path)?;
    let length = ris_io::seek(&mut file, SeekFrom::End(0))?;
    let mut bytes = vec![0u8; length as usize];
    ris_io::seek(&mut file, SeekFrom::Start(0))?;
    ris_io::read(&mut file, &mut bytes)?;
    RisGodAsset::deserialize(&bytes)
}

fn print_god_asset(god_asset: &RisGodAsset) {
    eprintln!();
    println!("RisGodAsset {{");
    println!("    default_vert_spv: {:?},", god_asset.default_vert_spv);
    println!("    default_frag_spv: {:?},", god_asset.default_frag_spv);
    println!("    imgui_vert_spv: {:?},", god_asset.imgui_vert_spv);
    println!("    imgui_frag_spv: {:?},", god_asset.imgui_frag_spv);
    println!(
        "    gizmo_segment_vert_spv: {:?},",
        god_asset.gizmo_segment_vert_spv
    );
    println!(
        "    gizmo_segment_frag_spv: {:?},",
        god_asset.gizmo_segment_frag_spv
    );
    println!(
        "    gizmo_test_vert_spv: {:?},",
        god_asset.gizmo_text_vert_spv
    );
    println!(
        "    gizmo_test_geom_spv: {:?},",
        god_asset.gizmo_text_geom_spv
    );
    println!(
        "    gizmo_test_frag_spv: {:?},",
        god_asset.gizmo_text_frag_spv
    );
    println!(
        "    debug_font_texture: {:?},",
        god_asset.debug_font_texture
    );
    println!("    texture: {:?},", god_asset.texture);
    println!("}}");
    eprintln!();
}

fn write_god_asset(god_asset: &RisGodAsset, path: impl AsRef<Path>) -> RisResult<()> {
    match god_asset.serialize() {
        Ok(bytes) => {
            let mut file = std::fs::File::create(path)?;
            ris_io::write(&mut file, &bytes)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}
