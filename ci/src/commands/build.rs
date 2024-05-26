use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::CiResult;
use crate::ICommand;

const AUTO_GENERATE_START: &str = "@@AUTO GENERATE START@@";
const AUTO_GENERATE_END: &str = "@@AUTO GENERATE END@@";

#[cfg(target_os = "windows")]
const EXE_NAME: &str = "ris_engine.exe";
#[cfg(target_os = "windows")]
const SDL2_NAME: &str = "SDL2.dll";

#[cfg(target_os = "linux")]
const EXE_NAME: &str = "ris_engine";

const ASSETS_NAME: &str = "ris_assets";

struct AutoGenerateParseData<'a> {
    is_generating: bool,
    to_replace: &'a str,
    is_multi_line: bool,
    multi_line: String,
    total_quotation_marks: usize,
    total_open_paranthesis: usize,
    total_close_paranthesis: usize,
    result: String,
}

pub struct Build;
impl ICommand for Build {
    fn args() -> String {
        String::new()
    }

    fn explanation() -> String {
        format!("Generates build info and compiles the workspace as a release ready package.")
    }

    fn run(_args: Vec<String>, target_dir: PathBuf) -> CiResult<()> {
        eprintln!("generating build info...");

        let root_dir = crate::util::get_root_dir()?;
        let build_info_path = root_dir
            .join("crates")
            .join("ris_data")
            .join("src")
            .join("info")
            .join("build_info.rs");

        let mut git_repo = String::new();
        let mut git_commit = String::new();
        let mut git_branch = String::new();
        let mut rustc_version = String::new();
        let mut rustup_toolchain = String::new();

        crate::cmd::run("git config --get remote.origin.url", Some(&mut git_repo))?;
        crate::cmd::run("git rev-parse HEAD", Some(&mut git_commit))?;
        crate::cmd::run("git rev-parse --abbrev-ref HEAD", Some(&mut git_branch))?;
        crate::cmd::run("rustc --version", Some(&mut rustc_version))?;
        crate::cmd::run("rustup show active-toolchain", Some(&mut rustup_toolchain))?;

        let build_date = chrono::Local::now().to_rfc3339();

        let git_repo = git_repo.trim();
        let git_commit = git_commit.trim();
        let git_branch = git_branch.trim();
        let rustc_version = rustc_version.trim();
        let rustup_toolchain = rustup_toolchain.trim();
        let build_date = build_date.trim();

        eprintln!("read previous build info... {:?}", build_info_path);

        let mut file_content = String::new();
        {
            let mut file = std::fs::File::open(&build_info_path)?;
            file.read_to_string(&mut file_content)?;
        }

        eprintln!("parse build info...");

        let mut data = AutoGenerateParseData {
            is_generating: false,
            to_replace: "",
            is_multi_line: false,
            multi_line: String::new(),
            total_quotation_marks: 0,
            total_open_paranthesis: 0,
            total_close_paranthesis: 0,
            result: String::new(),
        };

        for line in file_content.lines() {
            if line.contains(AUTO_GENERATE_START) {
                data.is_generating = true;
            }

            if line.contains(AUTO_GENERATE_END) {
                data.is_generating = false;
            }

            if data.is_generating {
                if line.contains("git_repo") {
                    data.to_replace = git_repo;
                    parse_multi_line(line, &mut data);
                } else if line.contains("git_commit") {
                    data.to_replace = git_commit;
                    parse_multi_line(line, &mut data);
                } else if line.contains("git_branch") {
                    data.to_replace = git_branch;
                    parse_multi_line(line, &mut data);
                } else if line.contains("rustc_version") {
                    data.to_replace = rustc_version;
                    parse_multi_line(line, &mut data);
                } else if line.contains("rustup_toolchain") {
                    data.to_replace = rustup_toolchain;
                    parse_multi_line(line, &mut data);
                } else if line.contains("build_date") {
                    data.to_replace = build_date;
                    parse_multi_line(line, &mut data);
                } else if data.is_multi_line {
                    parse_multi_line(line, &mut data);
                } else {
                    data.result += &format!("{}\n", line);
                }
            } else {
                data.result += &format!("{}\n", line);
            }
        }

        eprintln!("write new build info...");

        {
            let new_content = data.result.as_bytes();
            let mut file = std::fs::File::create(&build_info_path)?;
            file.write_all(new_content)?;
        }

        eprintln!("importing assets...");
        crate::cmd::run("cargo run -p ris_asset_compiler importall", None)?;
        eprintln!("compiling assets...");
        crate::cmd::run("cargo run -p ris_asset_compiler compile", None)?;

        eprintln!("compiling workspace...");
        crate::cmd::run("cargo build --release", None)?;

        crate::util::clean_or_create_dir(&target_dir)?;

        eprintln!("moving executable...");
        let src_exe_path = root_dir.join("target").join("release").join(EXE_NAME);
        let dst_exe_path = target_dir.join(EXE_NAME);
        std::fs::copy(src_exe_path, dst_exe_path)?;

        eprintln!("moving assets...");
        let src_asset_path = root_dir.join(ASSETS_NAME);
        let dst_asset_path = target_dir.join(ASSETS_NAME);
        std::fs::copy(src_asset_path, dst_asset_path)?;

        #[cfg(target_os = "windows")]
        {
            eprintln!("moving sdl2...");
            let mut src_sdl2_path = String::new();
            crate::cmd::run(
                &format!("where {}", SDL2_NAME),
                Some(&mut src_sdl2_path),
            )?;

            let src_sdl2_path = src_sdl2_path.trim();
            let dst_sdl2_path = target_dir.join(SDL2_NAME);
            eprintln!("attempting to copy {} from: {:?}", SDL2_NAME, src_sdl2_path);
            std::fs::copy(src_sdl2_path, dst_sdl2_path)?;
        }

        eprintln!("done! final build can be found under {:?}", target_dir);

        Ok(())
    }
}

fn parse_multi_line(line: &str, data: &mut AutoGenerateParseData) {
    for char in line.chars() {
        match char {
            '"' => data.total_quotation_marks += 1,
            '(' => data.total_open_paranthesis += 1,
            ')' => data.total_close_paranthesis += 1,
            _ => continue,
        }
    }

    let end_found = (data.total_quotation_marks > 0)
        && (data.total_quotation_marks % 2 == 0)
        && (data.total_open_paranthesis > 0)
        && (data.total_close_paranthesis > 0)
        && (data.total_open_paranthesis == data.total_close_paranthesis);

    if end_found {
        // end found! we can parse!
        data.multi_line += &format!("{}\n", line);

        let splits = data.multi_line.split('\"').collect::<Vec<_>>();
        let string1 = splits[0];
        let string2 = splits[splits.len() - 1];

        data.result += &format!("{}\"{}\"{}", string1, data.to_replace, string2,);

        data.multi_line = String::new();
        data.total_quotation_marks = 0;
        data.total_open_paranthesis = 0;
        data.total_close_paranthesis = 0;
        data.is_multi_line = false;
    } else {
        // end not found
        data.multi_line += &format!("{}\n", line);
        data.is_multi_line = true;
    }
}
