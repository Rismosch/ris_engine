use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_asset::AssetId;
use ris_asset::assets::ris_header::RisHeader;
use ris_asset::assets::ris_god_asset;
use ris_asset::assets::ris_god_asset::RisGodAsset;
use ris_data::info::args_info::DEFAULT_ASSETS_VALUE;
use ris_error::Extensions;
use ris_error::RisResult;

use crate::ExplanationLevel;
use crate::ICommand;

const ASSETS_PATH: &str = "assets path";

const CMD_EXIT: &str = "exit";
const CMD_HELP: &str = "help";
const CMD_WRITE: &str = "write";

pub struct GodAsset;

impl ICommand for GodAsset {
    fn args() -> String {
        format!("[{}]", ASSETS_PATH)
    }

    fn explanation(level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => {
                String::from("Modifies the god asset, the entry point of all assets.")
            },
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();
                explanation.push_str(&format!("Modifies the god asset, the entry point of all assets. It does so by locating it inside <{}>, deserializing it and presenting the user with a stdin command interface. Commands wont be committed, until the user explicitly writes their changes.\n", ASSETS_PATH));
                explanation.push('\n');
                explanation.push_str("args:\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", ASSETS_PATH));
                explanation.push_str("The path to the assets directory.\n");
                explanation.push_str(&format!("default: {}\n", DEFAULT_ASSETS_VALUE));

                explanation
            },
        }
    }

    fn run(args: Vec<String>, _target_dir: PathBuf) -> RisResult<()> {
        if args.len() > 3 {
            return crate::util::command_error(
                "too many args",
                "godasset",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            );
        }

        let assets_path = args
            .get(2)
            .map(|x| x.as_str())
            .unwrap_or(DEFAULT_ASSETS_VALUE);

        let god_asset_path = PathBuf::from(assets_path).join(ris_god_asset::PATH);

        let god_asset = match read_god_asset(&god_asset_path) {
            Ok(god_asset) => god_asset,
            Err(e) => {
                eprintln!("failed to read god asset: {}", e);
                eprintln!();
                eprintln!("do you want to create an empty god asset? [y/N]");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() != "y" {
                    return Ok(());
                } else {
                    RisGodAsset {
                        default_vert_spv: AssetId::Directory(String::new()),
                        default_frag_spv: AssetId::Directory(String::new()),
                        imgui_vert_spv: AssetId::Directory(String::new()),
                        imgui_frag_spv: AssetId::Directory(String::new()),
                        gizmo_segment_vert_spv: AssetId::Directory(String::new()),
                        gizmo_segment_geom_spv: AssetId::Directory(String::new()),
                        gizmo_segment_frag_spv: AssetId::Directory(String::new()),
                        gizmo_text_vert_spv: AssetId::Directory(String::new()),
                        gizmo_text_geom_spv: AssetId::Directory(String::new()),
                        gizmo_text_frag_spv: AssetId::Directory(String::new()),
                        debug_font_texture: AssetId::Directory(String::new()),
                        texture: AssetId::Directory(String::new()),
                    }
                }
            }
        };

        print_help();

        loop {
            eprintln!();
            eprintln!("enter a command:");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_lowercase().as_str() {
                CMD_EXIT => {
                    return Ok(());
                }
                CMD_HELP => {
                    eprintln!();
                    eprintln!("{}     exits this program without writing", CMD_EXIT);
                    eprintln!("{}     prints this menu", CMD_HELP);
                    eprintln!("{}    writes changes to the god asset and exits this program", CMD_WRITE);
                }
                CMD_WRITE => {
                    eprintln!("you are about to commit a write command. this action cannot be undone");
                    eprintln!("do you want to continue? [y/N]");

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    if input.trim().to_lowercase() == "y" {
                        eprintln!("todo");
                        return Ok(())
                    } 
                }
                _ => {
                    eprintln!("unkown command");
                    print_help();
                }
            }
        }
    }
}

fn read_god_asset(path: impl AsRef<Path>) -> RisResult<RisGodAsset> {
    let path = path.as_ref();
    eprintln!("read god_asset... \"{}\"", ris_io::path::to_str(path));
    let mut file = std::fs::File::open(path)?;
    let length = ris_io::seek(&mut file, SeekFrom::End(0))?;
    let mut bytes = vec![0u8; length as usize];
    ris_io::seek(&mut file, SeekFrom::Start(0))?;
    ris_io::read(&mut file, &mut bytes)?;
    RisGodAsset::load(&bytes)
}

fn print_help() {
    eprintln!("use `{}` to list all commands", CMD_HELP);
}
