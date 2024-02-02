use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use shaderc::CompilationArtifact;

use ris_error::RisResult;

pub const IN_EXT: &str = "glsl";
pub const OUT_EXT: &[&str] = &["vert.spirv", "frag.spirv"];

pub fn import(source: PathBuf, targets: Vec<PathBuf>) -> RisResult<()> {
    // read file
    let file =
        ris_error::unroll_option!(source.to_str(), "failed to convert {:?} to string", source,)?;

    let mut input = ris_error::unroll!(File::open(file), "failed to open file {:?}", file,)?;

    let file_size = ris_file::seek!(input, SeekFrom::End(0))?;
    ris_file::seek!(input, SeekFrom::Start(0))?;
    let mut file_content = vec![0u8; file_size as usize];
    ris_file::read!(input, file_content)?;
    let source_text = ris_error::unroll!(
        String::from_utf8(file_content),
        "failed to convert source to string",
    )?;

    // pre processor
    // init shaders
    let first_line = ris_error::unroll_option!(
        source_text.lines().next(),
        "failed to get first line of source file \"{}\"",
        file,
    )?;

    let magic = "#ris_glsl";
    preproc_assert(
        first_line.starts_with(magic),
        &format!("expected shader to start with \"{}\"", magic),
        file,
        0,
    )?;

    let splits = first_line.split(' ').collect::<Vec<_>>();
    let second_paramter = splits.get(1);
    if let Some(parameter) = second_paramter {
        if *parameter == "header" {
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

    let mut vert_glsl = Shader::new(ShaderKind::Vertex);
    let mut frag_glsl = Shader::new(ShaderKind::Fragment);

    for split in splits.iter().skip(2) {
        match *split {
            "vertex" => vert_glsl.init(version),
            "fragment" => frag_glsl.init(version),
            value => {
                return preproc_fail(&format!("invalid shaderkind value \"{}\"", value), file, 0)
            }
        }
    }

    // parse macros
    let mut current_region = Region::None;
    let mut already_included = Vec::new();
    let mut define_map = HashMap::new();
    let mut line = 0;
    for input_line in source_text.lines().skip(1) {
        line += 1;

        let splits = input_line.split(' ').collect::<Vec<_>>();
        let first_split = splits[0];

        match first_split {
            "#vertex" => current_region = Region::Shader(ShaderKind::Vertex),
            "#fragment" => current_region = Region::Shader(ShaderKind::Fragment),
            "#io" => {
                preproc_assert_arg_count(splits.len(), 3, file, line)?;
                let i = string_to_region_kind(splits[1], file, line)?;
                let o = string_to_region_kind(splits[2], file, line)?;

                current_region = Region::IO(i, o);
            }
            "#define" => {
                add_define(&mut define_map, &splits, file, line)?;
            }
            "#include" => {
                let file_path = PathBuf::from(file);
                let root_dir = ris_error::unroll_option!(
                    file_path.parent(),
                    "{:?} has no parent",
                    file_path,
                )?;

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
                    &mut vert_glsl,
                    &mut frag_glsl,
                    &define_map,
                )?;
            }
            _ => {
                add_content(
                    input_line,
                    &current_region,
                    &mut vert_glsl,
                    &mut frag_glsl,
                    &define_map,
                )?;
            }
        }
    }

    // compile to spirv
    let compiler = ris_error::unroll_option!(
        shaderc::Compiler::new(),
        "failed to initialize shaderc compiler"
    )?;
    let mut options = ris_error::unroll_option!(
        shaderc::CompileOptions::new(),
        "failed to initialize shaderc options"
    )?;
    options.set_warnings_as_errors();
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);

    let mut artifacts = Vec::new();

    let vert_artifact = vert_glsl.compile(file, &compiler, &options)?;
    artifacts.push(vert_artifact);

    let frag_artifact = frag_glsl.compile(file, &compiler, &options)?;
    artifacts.push(frag_artifact);

    // save to file
    debug_assert_eq!(artifacts.len(), targets.len());
    for i in 0..artifacts.len() {
        let artifact = &artifacts[i];
        let target = &targets[i];

        if let Some(artifact) = artifact {
            let mut output = crate::asset_importer::create_file(target)?;
            let bytes = artifact.as_binary_u8();

            ris_file::write!(&mut output, bytes)?;
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Region {
    None,
    Shader(ShaderKind),
    IO(ShaderKind, ShaderKind),
}

#[derive(Debug, PartialEq, Eq)]
enum ShaderKind {
    Vertex,
    Fragment,
}

impl std::fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderKind::Vertex => write!(f, "vertex"),
            ShaderKind::Fragment => write!(f, "fragment"),
        }
    }
}

struct Shader {
    kind: ShaderKind,
    source: Option<String>,
}

impl Shader {
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
        compiler: &shaderc::Compiler,
        options: &shaderc::CompileOptions,
    ) -> RisResult<Option<CompilationArtifact>> {
        let source = match self.source {
            Some(source) => source,
            None => return Ok(None),
        };

        let file_path = PathBuf::from(file);
        let file_stem =
            ris_error::unroll_option!(file_path.file_stem(), "file {:?} has no stem", file_path,)?;
        let file_stem =
            ris_error::unroll_option!(file_stem.to_str(), "failed to convert path to string",)?;

        let extension = match self.kind {
            ShaderKind::Vertex => OUT_EXT[0],
            ShaderKind::Fragment => OUT_EXT[1],
        };

        let file = format!("{}.{}", file_stem, extension);

        let artifact = ris_error::unroll!(
            compiler.compile_into_spirv(
                &source,
                shaderc::ShaderKind::InferFromSource,
                &file,
                "main",
                Some(options),
            ),
            "failed to compile shader {}",
            file,
        )?;

        Ok(Some(artifact))
    }
}

fn string_to_region_kind(value: &str, file: &str, line: usize) -> RisResult<ShaderKind> {
    match value {
        "vertex" => Ok(ShaderKind::Vertex),
        "fragment" => Ok(ShaderKind::Fragment),
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

    let file = ris_error::unroll_option!(
        include_path.to_str(),
        "failed to convert {:?} to a string",
        include_path,
    )?;

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
    let mut include_file = ris_error::unroll!(
        std::fs::File::open(&include_path),
        "failed to open {:?}",
        include_path,
    )?;

    let mut file_content = String::new();
    ris_error::unroll!(
        include_file.read_to_string(&mut file_content),
        "failed to read {:?}",
        include_path,
    )?;

    let first_line = ris_error::unroll_option!(
        file_content.lines().next(),
        "failed to get the first line of {:?}",
        include_path,
    )?;

    let magic = "#ris_glsl header";
    preproc_assert(
        first_line == magic,
        &format!("included headers must start with {}", magic),
        file,
        0,
    )?;

    // parse content
    let mut result = String::new();

    let mut line = 0;
    for input_line in file_content.lines().skip(1) {
        line += 1;

        let splits = input_line.split(' ').collect::<Vec<_>>();
        let first_split = splits[0];

        match first_split {
            "#define" => {
                add_define(define_map, &splits, file, line)?;
            }
            "#include" => {
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
                if input_line.is_empty() {
                    continue;
                }

                result.push('\n');
                result.push_str(input_line);
            }
        }
    }

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
    vert: &mut Shader,
    frag: &mut Shader,
    define_map: &HashMap<String, String>,
) -> RisResult<()> {
    if content.is_empty() {
        return Ok(());
    }

    match &current_region {
        Region::None => {
            vert.push(content, define_map);
            frag.push(content, define_map);
        }
        Region::Shader(ShaderKind::Vertex) => vert.push(content, define_map),
        Region::Shader(ShaderKind::Fragment) => frag.push(content, define_map),
        Region::IO(ShaderKind::Vertex, ShaderKind::Fragment) => {
            let vert_line = content.replace("IN_OUT", "out");
            let frag_line = content.replace("IN_OUT", "in");

            vert.push(&vert_line, define_map);
            frag.push(&frag_line, define_map);
        }
        region => ris_error::new_result!("invalid region: {:?}", region)?,
    };

    Ok(())
}
