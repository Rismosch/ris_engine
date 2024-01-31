use std::collections::HashMap;

use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use ris_error::RisResult;

pub const IN_EXT: &str = "glsl";
pub const OUT_EXT: &[&str] = &["vert.spirv", "frag.spirv"];

pub fn import(
    file: &str,
    input: &mut (impl Read + Seek),
    output: &mut [impl Write + Seek],
) -> RisResult<()> {
    // read file
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
    if !first_line.starts_with(magic) {
        return preproc_fail(
            &format!("expected shader to start with \"{}\"", magic),
            file,
            0,
        );
    }

    let mut vert_glsl = Shader::new(ShaderKind::Vertex);
    let mut frag_glsl = Shader::new(ShaderKind::Fragment);

    let splits = first_line.split(' ').collect::<Vec<_>>();
    preproc_assert(
        splits.len() > 2,
        "ris_glsl must have 2 or more argument: one glsl version and which shaders this file contains",
        file,
        0,
    )?;

    let version = splits[1];

    for split in splits.iter().skip(2) {
        match *split {
            "vertex" => vert_glsl.init(version),
            "fragment" => frag_glsl.init(version),
            value => return preproc_fail(
                &format!("invalid shaderkind value \"{}\"",value),
                file,
                0,
            ),
        }
    }

    // parse macros
    let mut current_region = Region::None;
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
            },
            "#define" => {
                preproc_assert(
                    splits.len() > 2,
                    "to few arguments for #define",
                    file,
                    line,
                )?;

                let key = splits[1].to_string();
                let value = splits.iter().skip(2).cloned().collect::<Vec<_>>().join(" ");

                let prev = define_map.insert(key.clone(), value);
                preproc_assert(
                    prev.is_none(),
                    &format!("define key \"{}\" was already defined", key),
                    file,
                    line,
                )?;
            },
            "#include" => {
                preproc_assert_arg_count(splits.len(), 2, file, line)?;
                let include = splits[1].to_string();

                println!("handle include {}", include);

                //includes.insert(to_include);
            },
            _ => {
                if input_line.is_empty() {
                    continue;
                }

                match &current_region {
                    Region::None => preproc_fail("encountered code outside a dedicated region", file, line)?,
                    Region::Shader(ShaderKind::Vertex) => vert_glsl.push(input_line, &define_map),
                    Region::Shader(ShaderKind::Fragment) => frag_glsl.push(input_line, &define_map),
                    Region::IO(ShaderKind::Vertex, ShaderKind::Fragment) => {
                        let vert_line = input_line.replace("IN_OUT", "out");
                        let frag_line = input_line.replace("IN_OUT", "in");

                        vert_glsl.push(&vert_line, &define_map);
                        frag_glsl.push(&frag_line, &define_map);
                    },
                    region => ris_error::new_result!("invalid region: {:?}", region)?,
                };
            },
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

    vert_glsl.compile(file, &compiler, &options, &mut output[0])?;
    frag_glsl.compile(file, &compiler, &options, &mut output[1])?;

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

struct Shader{
    kind: ShaderKind,
    source: Option<String>,
}

impl Shader {
    pub fn new(kind: ShaderKind) -> Self {
        Self {
            kind,
            source: None,
        }
    }

    pub fn init(&mut self, version: &str) {
        self.source = Some(format!(
                "#version {}\n#pragma shader_stage({})",
                version,
                self.kind,
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
        output: &mut (impl Write + Seek),
    ) -> RisResult<usize> {
        let source = match self.source {
            Some(source) => source,
            None => return Ok(0),
        };

        let file_path = PathBuf::from(file);
        let file_stem = ris_error::unroll_option!(
            file_path.file_stem(),
            "file {:?} has no stem",
            file_path,
        )?;
        let file_stem = ris_error::unroll_option!(
            file_stem.to_str(),
            "failed to convert path to string",
        )?;

        let extension = match self.kind {
            ShaderKind::Vertex => OUT_EXT[0],
            ShaderKind::Fragment => OUT_EXT[1],
        };

        let file = format!("{}.{}",file_stem, extension);

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
        let bytes = artifact.as_binary_u8();


        ris_file::write!(output, bytes)
    }
}

fn string_to_region_kind(value: &str, file: &str, line: usize) -> RisResult<ShaderKind> {
    match value {
        "vertex" => Ok(ShaderKind::Vertex),
        "fragment" => Ok(ShaderKind::Fragment),
        value => preproc_fail(&format!("invalid region kind: {}", value), file, line),
    }
}

fn preproc_assert_arg_count(actual: usize, expected: usize, file: &str, line: usize) -> RisResult<()> {
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
    ris_error::new_result!(
        "preproc assert failed: {} in {}:{}",
        message,
        file,
        line,
    )
}

