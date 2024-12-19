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

const CMD_CD: &str = "cd";
const CMD_EXIT: &str = "exit";
const CMD_HELP: &str = "help";
const CMD_LS: &str = "ls";
const CMD_PRINT: &str = "print";
const CMD_SET: &str = "set";
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

        let mut god_asset = match read_god_asset(&god_asset_path) {
            Ok(god_asset) => god_asset,
            Err(e) => {
                eprintln!("failed to read god asset: {}", e);

                let new_god_asset = RisGodAsset {
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
                };

                eprintln!();
                eprintln!("a new god asset has been generated");
                eprintln!();

                new_god_asset
            }
        };

        print_help();
        let mut current_dir = PathBuf::from(assets_path);

        loop {
            eprintln!();
            eprintln!("{} ~", ris_io::path::to_str(&current_dir));

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let trimmed = input.trim().to_lowercase();
            let cmd = trimmed
                .split(' ')
                .collect::<Vec<_>>();

            match cmd.as_slice() {
                &[CMD_CD, dir] => {
                    match dir {
                        "." => continue,
                        ".." => {
                            if let Some(new_dir) = current_dir.clone().parent() {
                                if !new_dir.exists() {
                                    eprintln!("directory does not exist");
                                    continue;
                                }

                                current_dir = new_dir.to_path_buf();
                            };
                        },
                        dir => {
                            let new_dir = current_dir.clone().join(dir);
                            if !new_dir.is_dir() {
                                eprintln!("not a directory or does not exist");
                                continue;
                            }

                            current_dir = new_dir;
                        }
                    }

                },
                &[CMD_EXIT] => {
                    eprintln!();
                    eprintln!("your changes will be lost.");
                    eprintln!("do you to exit anyway? [y/N]");

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;

                    if input.trim().to_lowercase().as_str() == "y" {
                        return Ok(());
                    }
                },
                &[CMD_HELP] => {
                    eprintln!();
                    eprintln!("{} <dir>               changes the directory to <dir>", CMD_CD);
                    eprintln!("{}                   exits this program without writing", CMD_EXIT);
                    eprintln!("{}                   prints this menu", CMD_HELP);
                    eprintln!("{}                     lists the entries in the current directory", CMD_LS);
                    eprintln!("{}                  prints the current state of the god asset", CMD_PRINT);
                    eprintln!("{} <field> <value>    sets the <field> to <value> of the god asset", CMD_SET);
                    eprintln!("{}                  writes changes to the god asset", CMD_WRITE);
                },
                &[CMD_LS] => {
                    let entries = std::fs::read_dir(&current_dir)?;
                    for entry in entries {
                        let entry = entry?;
                        let metadata = entry.metadata()?;
                        let file_name = entry.file_name();
                        let file_name = file_name.to_str().into_ris_error()?;

                        if metadata.is_file() {
                            eprintln!("file    {}", file_name);
                        } else if metadata.is_dir() {
                            eprintln!("dir     {}", file_name);
                        } else if metadata.is_symlink() {
                            eprintln!("symlink {}", file_name);
                        } else {
                            eprintln!("unkown  {}", file_name);
                        }
                    }
                },
                &[CMD_PRINT] => {
                    eprintln!();
                    eprintln!("RisGodAsset {{");
                    eprintln!("    default_vert_spv: {:?},", god_asset.default_vert_spv);
                    eprintln!("    default_frag_spv: {:?},", god_asset.default_frag_spv);
                    eprintln!("    imgui_vert_spv: {:?},", god_asset.imgui_vert_spv);
                    eprintln!("    imgui_frag_spv: {:?},", god_asset.imgui_frag_spv);
                    eprintln!("    gizmo_segment_vert_spv: {:?},", god_asset.gizmo_segment_vert_spv);
                    eprintln!("    gizmo_segment_geom_spv: {:?},", god_asset.gizmo_segment_geom_spv);
                    eprintln!("    gizmo_segment_frag_spv: {:?},", god_asset.gizmo_segment_frag_spv);
                    eprintln!("    gizmo_test_vert_spv: {:?},", god_asset.gizmo_text_vert_spv);
                    eprintln!("    gizmo_test_geom_spv: {:?},", god_asset.gizmo_text_geom_spv);
                    eprintln!("    gizmo_test_frag_spv: {:?},", god_asset.gizmo_text_frag_spv);
                    eprintln!("    debug_font_texture: {:?},", god_asset.debug_font_texture);
                    eprintln!("    texture: {:?},", god_asset.texture);
                    eprintln!("}}");
                },
                &[CMD_SET, field, value] => {
                    let path = current_dir.clone().join(value);
                    if !path.is_file() {
                        eprintln!("value is not a file or doesn't exist");
                        continue;
                    }

                    let path_without_root = PathBuf::from_iter(path.components().skip(1));
                    let to_set = ris_io::path::to_str(path_without_root).replace('\\', "/");

                    match field.trim().to_lowercase().as_str() {
                        "default_vert_spv" => god_asset.default_vert_spv = AssetId::Directory(to_set),
                        "default_frag_spv" => god_asset.default_frag_spv = AssetId::Directory(to_set),
                        "imgui_vert_spv" => god_asset.imgui_vert_spv = AssetId::Directory(to_set),
                        "imgui_frag_spv" => god_asset.imgui_frag_spv = AssetId::Directory(to_set),
                        "gizmo_segment_vert_spv" => god_asset.gizmo_segment_vert_spv = AssetId::Directory(to_set),
                        "gizmo_segment_geom_spv" => god_asset.gizmo_segment_geom_spv = AssetId::Directory(to_set),
                        "gizmo_segment_frag_spv" => god_asset.gizmo_segment_frag_spv = AssetId::Directory(to_set),
                        "gizmo_text_vert_spv" => god_asset.gizmo_text_vert_spv = AssetId::Directory(to_set),
                        "gizmo_text_geom_spv" => god_asset.gizmo_text_geom_spv = AssetId::Directory(to_set),
                        "gizmo_text_frag_spv" => god_asset.gizmo_text_frag_spv = AssetId::Directory(to_set),
                        "debug_font_texture" => god_asset.debug_font_texture = AssetId::Directory(to_set),
                        "texture" => god_asset.texture = AssetId::Directory(to_set),
                        _ => eprintln!("unkown field"),
                    }
                },
                &[CMD_WRITE] => {
                    if let Err(e) = write_god_asset(&god_asset, &god_asset_path) {
                        eprintln!("failed to write god asset: {}", e);
                    }
                },
                _ => {
                    eprintln!("invalid command")
                },
            }
        }
    }
}

fn read_god_asset(path: impl AsRef<Path>) -> RisResult<RisGodAsset> {
    let path = path.as_ref();
    eprintln!("reading god_asset... \"{}\"", ris_io::path::to_str(path));
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

fn write_god_asset(god_asset: &RisGodAsset, path: impl AsRef<Path>) -> RisResult<()> {
    match god_asset.serialize() {
        Ok(bytes) => {
            let mut file = std::fs::File::create(path)?;
            let bytes = god_asset.serialize()?;
            ris_io::write(&mut file, &bytes)?;
            Ok(())
        },
        Err(e) => Err(e),
    }
}
