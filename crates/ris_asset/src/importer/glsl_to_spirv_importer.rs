use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use shaderc::CompilationArtifact;

use ris_error::Extensions;
use ris_error::RisResult;

pub const IN_EXT: &str = "glsl";
pub const OUT_EXT: &[&str] = &["vert.spv", "geom.spv", "frag.spv"];

const PATH_PREFIX: &str = "assets/__raw/shaders";
const NAME: &str = "glsl_to_spirv_importer";

const MAGIC: &str = "#ris_glsl";
const HEADER: &str = "header";
const VERTEX: &str = "vertex";
const GEOMETRY: &str = "geometry";
const FRAGMENT: &str = "fragment";

const MACRO_VERTEX: &str = "#vertex";
const MACRO_GEOMETRY: &str = "#geometry";
const MACRO_FRAGMENT: &str = "#fragment";
const MACRO_IO: &str = "#io";
const MACRO_DEFINE: &str = "#define";
const MACRO_INCLUDE: &str = "#include";

const VERT: &str = "vert";
const GEOM: &str = "geom";
const FRAG: &str = "frag";

const IN_OUT: &str = "IN_OUT";
const IN: &str = "in";
const OUT: &str = "out";

#[derive(Debug, PartialEq, Eq)]
enum Region {
    None,
    Shader(ShaderKind),
    IO(ShaderKind, ShaderKind),
}

#[derive(Debug, PartialEq, Eq)]
enum ShaderKind {
    Vertex,
    Geometry,
    Fragment,
}

impl std::fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderKind::Vertex => write!(f, "{}", VERTEX),
            ShaderKind::Geometry => write!(f, "{}", GEOMETRY),
            ShaderKind::Fragment => write!(f, "{}", FRAGMENT),
        }
    }
}

struct ShaderStage {
    kind: ShaderKind,
    source: Option<String>,
}

struct Shader {
    vert: ShaderStage,
    geom: ShaderStage,
    frag: ShaderStage,
}

impl ShaderStage {
    pub fn new(kind: ShaderKind) -> Self {
        Self { kind, source: None }
    }

    pub fn init(&mut self, version: &str) {
        self.source = Some(format!(
            "#version {}\n#pragma shader_stage({})",
            version, self.kind,
        ));
    }

    pub fn push(&mut self, line: &str, define_map: &HashMap<String, String>) {
        if let Some(shader) = &mut self.source {
            shader.push('\n');

            let mut to_push = line.to_string();

            for (key, value) in define_map {
                to_push = to_push.replace(key, value);
            }

            shader.push_str(&to_push);
        }
    }

    pub fn compile(
        self,
        file: &str,
        temp_dir: Option<&Path>,
        compiler: &shaderc::Compiler,
        options: &shaderc::CompileOptions,
    ) -> RisResult<Option<CompilationArtifact>> {
        let source = match self.source {
            Some(source) => source,
            None => return Ok(None),
        };

        let file_path = PathBuf::from(file);
        let file_stem = file_path.file_stem().into_ris_error()?;
        let file_stem = file_stem.to_str().into_ris_error()?;
        let file_extension = file_path.extension().into_ris_error()?;
        let file_extension = file_extension.to_str().into_ris_error()?;

        let shader_extension = match self.kind {
            ShaderKind::Vertex => VERT,
            ShaderKind::Geometry => GEOM,
            ShaderKind::Fragment => FRAG,
        };

        let file = format!("{}.{}.{}", file_stem, shader_extension, file_extension);

        if let Some(temp_dir) = temp_dir {
            let parent = file_path.parent().into_ris_error()?;
            let parent = parent.to_str().into_ris_error()?;
            let parent = parent.replace('\\', "/");
            let parent = match parent.strip_prefix(PATH_PREFIX) {
                Some(parent) => parent.to_string(),
                None => parent,
            };
            let parent = PathBuf::from(parent);

            let dir = temp_dir.join(parent).join(NAME);
            let temp_file_path = dir.join(file.clone());

            std::fs::create_dir_all(dir)?;
            let mut temp_file = std::fs::File::create(&temp_file_path)?;
            ris_io::write(&mut temp_file, source.as_bytes())?;

            ris_log::trace!(
                "saved transpiled shader to: \"{}\"",
                ris_io::path::to_str(temp_file_path),
            );
        }

        let artifact = compiler
            .compile_into_spirv(
                &source,
                shaderc::ShaderKind::InferFromSource,
                &file,
                "main",
                Some(options),
            )
            .map_err(|e| {
                let mut log_source = String::new();
                for (i, line) in source.lines().enumerate() {
                    log_source.push_str(&format!("{:>8} {}\n", i + 1, line));
                }

                let base_message = format!("failed to compile shader \"{}\"", file);

                ris_log::error!("{}\n\nsource:\n{}\nerror:\n{}", base_message, log_source, e,);

                ris_error::new!("{}. check log for more infos.", base_message)
            })?;

        Ok(Some(artifact))
    }
}

pub fn import(source: PathBuf, targets: Vec<PathBuf>, temp_dir: Option<&Path>) -> RisResult<()> {
    // read file
    let file = source.to_str().into_ris_error()?;

    let mut source = File::open(file)?;
    let f = &mut source;

    let file_size = ris_io::seek(f, SeekFrom::End(0))?;
    let mut file_content = vec![0u8; file_size as usize];
    ris_io::seek(f, SeekFrom::Start(0))?;
    ris_io::read(f, &mut file_content)?;
    let source_text = String::from_utf8(file_content)?;

    // pre processor
    // init shaders
    let first_line = source_text.lines().next().into_ris_error()?;

    preproc_assert(
        first_line.starts_with(MAGIC),
        &format!("expected shader to start with \"{}\"", MAGIC),
        file,
        0,
    )?;

    let splits = first_line.split(' ').collect::<Vec<_>>();
    let second_paramter = splits.get(1);
    if let Some(parameter) = second_paramter {
        if *parameter == HEADER {
            return Ok(());
        }
    }

    preproc_assert(
        splits.len() > 2,
        "ris_glsl must have 2 or more argument: one glsl version and which shaders this file contains",
        file,
        0,
    )?;

    let version = splits[1];

    let mut shader = Shader {
        vert: ShaderStage::new(ShaderKind::Vertex),
        geom: ShaderStage::new(ShaderKind::Geometry),
        frag: ShaderStage::new(ShaderKind::Fragment),
    };

    for split in splits.iter().skip(2) {
        match *split {
            VERTEX => shader.vert.init(version),
            GEOMETRY => shader.geom.init(version),
            FRAGMENT => shader.frag.init(version),
            value => {
                return preproc_fail(&format!("invalid shaderkind value \"{}\"", value), file, 0)
            }
        }
    }

    // parse macros
    let mut current_region = Region::None;
    let mut already_included = Vec::new();
    let mut define_map = HashMap::new();
    let mut line = 1; // start at 1, because we skip the first line
    for input_line in source_text.lines().skip(1) {
        line += 1;

        let splits = input_line.split(' ').collect::<Vec<_>>();
        let first_split = splits[0];

        match first_split {
            MACRO_VERTEX => current_region = Region::Shader(ShaderKind::Vertex),
            MACRO_GEOMETRY => current_region = Region::Shader(ShaderKind::Geometry),
            MACRO_FRAGMENT => current_region = Region::Shader(ShaderKind::Fragment),
            MACRO_IO => {
                preproc_assert_arg_count(splits.len(), 3, file, line)?;
                let i = string_to_region_kind(splits[1], file, line)?;
                let o = string_to_region_kind(splits[2], file, line)?;

                current_region = Region::IO(i, o);
            }
            MACRO_DEFINE => {
                add_define(&mut define_map, &splits, file, line)?;
            }
            MACRO_INCLUDE => {
                let file_path = PathBuf::from(file);
                let root_dir = file_path.parent().into_ris_error()?;

                let mut dependency_history = Vec::new();
                dependency_history.push(file_path.clone());

                let include_content = resolve_include(
                    &splits,
                    root_dir,
                    &mut already_included,
                    &mut dependency_history,
                    &mut define_map,
                    file,
                    line,
                )?;

                add_content(
                    &include_content,
                    &current_region,
                    &mut shader,
                    &define_map,
                    file,
                    line,
                )?;
            }
            _ => {
                add_content(
                    input_line,
                    &current_region,
                    &mut shader,
                    &define_map,
                    file,
                    line,
                )?;
            }
        }
    }

    // compile to spirv
    let compiler = shaderc::Compiler::new().into_ris_error()?;
    let mut options = shaderc::CompileOptions::new().into_ris_error()?;
    options.set_warnings_as_errors();
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);

    let mut artifacts = Vec::new();

    let vert_artifact = shader.vert.compile(file, temp_dir, &compiler, &options)?;
    artifacts.push(vert_artifact);

    let geom_artifact = shader.geom.compile(file, temp_dir, &compiler, &options)?;
    artifacts.push(geom_artifact);

    let frag_artifact = shader.frag.compile(file, temp_dir, &compiler, &options)?;
    artifacts.push(frag_artifact);

    // save to file
    debug_assert_eq!(artifacts.len(), targets.len());
    for i in 0..artifacts.len() {
        let artifact = &artifacts[i];
        let target = &targets[i];

        if let Some(artifact) = artifact {
            let mut output = crate::asset_importer::create_file(target)?;
            let bytes = artifact.as_binary_u8();

            ris_io::write(&mut output, bytes)?;
        }
    }

    Ok(())
}

fn string_to_region_kind(value: &str, file: &str, line: usize) -> RisResult<ShaderKind> {
    match value {
        VERTEX => Ok(ShaderKind::Vertex),
        GEOMETRY => Ok(ShaderKind::Geometry),
        FRAGMENT => Ok(ShaderKind::Fragment),
        value => preproc_fail(&format!("invalid region kind: {}", value), file, line),
    }
}

fn preproc_assert_arg_count(
    actual: usize,
    expected: usize,
    file: &str,
    line: usize,
) -> RisResult<()> {
    preproc_assert(
        actual == expected,
        "incorrect number of arguments",
        file,
        line,
    )
}

fn preproc_assert(value: bool, message: &str, file: &str, line: usize) -> RisResult<()> {
    if value {
        Ok(())
    } else {
        preproc_fail(message, file, line)
    }
}

fn preproc_fail<T>(message: &str, file: &str, line: usize) -> RisResult<T> {
    ris_error::new_result!("preproc assert failed: {} in {}:{}", message, file, line,)
}

fn resolve_include(
    splits: &[&str],
    root_dir: &Path,
    already_included: &mut Vec<PathBuf>,
    dependency_history: &mut Vec<PathBuf>,
    define_map: &mut HashMap<String, String>,
    file: &str,
    line: usize,
) -> RisResult<String> {
    // create path
    preproc_assert_arg_count(splits.len(), 2, file, line)?;
    let to_include = splits[1];

    let mut include_path = PathBuf::new();
    include_path.push(root_dir);
    include_path.push(to_include);

    for path in already_included.iter() {
        if include_path == *path {
            return Ok(String::new());
        }
    }

    already_included.push(include_path.clone());

    let file = include_path.to_str().into_ris_error()?;

    // check for circular dependency
    if dependency_history.iter().any(|x| *x == include_path) {
        let mut error_message = String::from("circular dependency detected. history: \n");
        error_message.push_str(&format!("0 {:?}", include_path));

        for (i, dependency) in dependency_history.iter().rev().enumerate() {
            error_message.push_str(&format!("\n{} {:?}", i + 1, dependency));
        }

        return ris_error::new_result!("{}", error_message);
    }

    dependency_history.push(include_path.clone());

    // read file
    let mut include_file = std::fs::File::open(&include_path)?;

    let mut file_content = String::new();
    include_file.read_to_string(&mut file_content)?;

    let first_line = file_content.lines().next().into_ris_error()?;

    let magic = "#ris_glsl header";
    preproc_assert(
        first_line == magic,
        &format!("included headers must start with {}", magic),
        file,
        0,
    )?;

    // parse content
    let include_path_comment = include_path.to_str().into_ris_error()?.replace('\\', "/");
    let mut result = format!("//////// INCLUDE {}", include_path_comment);

    let mut line = 0;
    for input_line in file_content.lines().skip(1) {
        line += 1;

        let splits = input_line.split(' ').collect::<Vec<_>>();
        let first_split = splits[0];

        match first_split {
            MACRO_DEFINE => {
                add_define(define_map, &splits, file, line)?;
            }
            MACRO_INCLUDE => {
                let include_content = resolve_include(
                    &splits,
                    root_dir,
                    already_included,
                    dependency_history,
                    define_map,
                    file,
                    line,
                )?;

                result.push('\n');
                result.push_str(&include_content);
            }
            _ => {
                result.push('\n');
                result.push_str(input_line);
            }
        }
    }

    result.push_str(&format!("\n//////// END {}", include_path_comment));
    Ok(result)
}

fn add_define(
    define_map: &mut HashMap<String, String>,
    splits: &[&str],
    file: &str,
    line: usize,
) -> RisResult<()> {
    preproc_assert(splits.len() > 2, "to few arguments for #define", file, line)?;

    let key = splits[1].to_string();
    let value = splits.iter().skip(2).cloned().collect::<Vec<_>>().join(" ");

    let prev = define_map.insert(key.clone(), value);
    preproc_assert(
        prev.is_none(),
        &format!("define key \"{}\" was already defined", key),
        file,
        line,
    )?;

    Ok(())
}

fn add_content(
    content: &str,
    current_region: &Region,
    shader: &mut Shader,
    define_map: &HashMap<String, String>,
    file: &str,
    line: usize,
) -> RisResult<()> {
    match &current_region {
        Region::None => {
            shader.vert.push(content, define_map);
            shader.geom.push(content, define_map);
            shader.frag.push(content, define_map);
        }
        Region::Shader(ShaderKind::Vertex) => shader.vert.push(content, define_map),
        Region::Shader(ShaderKind::Geometry) => shader.geom.push(content, define_map),
        Region::Shader(ShaderKind::Fragment) => shader.frag.push(content, define_map),
        Region::IO(ShaderKind::Vertex, ShaderKind::Fragment) => {
            let vert_line = resolve_in_out(content, OUT, false);
            let frag_line = resolve_in_out(content, IN, false);

            shader.vert.push(&vert_line, define_map);
            shader.frag.push(&frag_line, define_map);
        }
        Region::IO(ShaderKind::Vertex, ShaderKind::Geometry) => {
            let vert_line = resolve_in_out(content, OUT, false);
            let geom_line = resolve_in_out(content, IN, true);

            shader.vert.push(&vert_line, define_map);
            shader.geom.push(&geom_line, define_map);
        }
        Region::IO(ShaderKind::Geometry, ShaderKind::Fragment) => {
            let geom_line = resolve_in_out(content, OUT, false);
            let frag_line = resolve_in_out(content, IN, false);

            shader.geom.push(&geom_line, define_map);
            shader.frag.push(&frag_line, define_map);
        }
        region => preproc_fail(&format!("invalid region: {:?}", region), file, line)?,
    };

    Ok(())
}

fn resolve_in_out(line: &str, token: &str, add_array: bool) -> String {
    let mut line = line.replace(IN_OUT, token);

    if add_array {
        if let Some(index) = line.find(';') {
            line.insert_str(index, "[]");
        }
    }

    line
}
