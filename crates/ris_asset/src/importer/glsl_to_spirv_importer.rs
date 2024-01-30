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
    let PreProcOutput { vert_glsl, frag_glsl } = pre_processor(source_text, file)?;

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

    compile_and_write_to_stream(
        &compiler,
        &options,
        file,
        ShaderKind::Vertex,
        &vert_glsl,
        &mut output[0],
    )?;

    compile_and_write_to_stream(
        &compiler,
        &options,
        file,
        ShaderKind::Fragment,
        &frag_glsl,
        &mut output[1],
    )?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum ShaderKind {
    Vertex,
    Fragment,
}

#[derive(Debug, PartialEq, Eq)]
enum Region {
    None,
    Layout(ShaderKind),
    LayoutIO(ShaderKind, ShaderKind),
    Entry(ShaderKind),
}

impl std::fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderKind::Vertex => write!(f, "vertex"),
            ShaderKind::Fragment => write!(f, "fragment"),
        }
    }
}

struct PreProcOutput {
    vert_glsl: String,
    frag_glsl: String,
}

fn pre_processor(input: String, file: &str) -> RisResult<PreProcOutput> {
    let mut glsl_version = None;
    let mut vert_layout = Vec::new();
    let mut frag_layout = Vec::new();
    let mut vert_io_frag = Vec::new();
    let mut vert_entry = Vec::new();
    let mut frag_entry = Vec::new();

    let mut current_region = Region::None;

    let mut line = 0;
    for input_line in input.lines() {
        line += 1;

        let splits: Vec<&str> = input_line.split(' ').collect();
        let first_split = splits[0];

        match first_split {
            "#glsl_version" => {
                preproc_assert_arg_count(splits.len(), 2, file, line)?;
                glsl_version = Some(splits[1].to_string());
            },
            "#layout" => {
                match splits.len() {
                    2 => {
                        let region_kind = string_to_region_kind(splits[1], file, line)?;
                        current_region = Region::Layout(region_kind);
                    },
                    4 => {
                        preproc_assert(splits[1] == "io", "invalid argument", file, line)?;
                        let i = string_to_region_kind(splits[2], file, line)?;
                        let o = string_to_region_kind(splits[3], file, line)?;
                        current_region = Region::LayoutIO(i, o);
                    },
                    _ => preproc_assert_arg_count(0, 1, file, line)?,
                }
            },
            "#entry" => {
                preproc_assert_arg_count(splits.len(), 2, file, line)?;
                let region_kind = string_to_region_kind(splits[1], file, line)?;
                current_region = Region::Entry(region_kind);
            },
            //"#const" => (),
            //"#include" => (),
            //"#once" => (),
            _ => {
                if input_line.is_empty() {
                    continue;
                }

                preproc_assert(
                    current_region != Region::None,
                    "encountered code outside a dedicated region",
                    file,
                    line,
                )?;

                match &current_region {
                    Region::None => preproc_fail("encountered code outside a dedicated region", file, line)?,
                    Region::Layout(ShaderKind::Vertex) => vert_layout.push(input_line),
                    Region::Layout(ShaderKind::Fragment) => frag_layout.push(input_line),
                    Region::LayoutIO(ShaderKind::Vertex, ShaderKind::Fragment) => vert_io_frag.push(input_line),
                    Region::Entry(ShaderKind::Vertex) => vert_entry.push(input_line),
                    Region::Entry(ShaderKind::Fragment) => frag_entry.push(input_line),
                    region => ris_error::new_result!("invalid region: {:?}", region)?,
                };
            },
        }
    }

    let version = match glsl_version {
        Some(version) => version,
        None => return preproc_fail("shader contains no glsl_version", file, 0),
    };

    let vert_glsl = build_glsl(
        &version,
        ShaderKind::Vertex,
        &vert_layout,
        None,
        Some(&vert_io_frag),
        &vert_entry,
    );

    let frag_glsl = build_glsl(
        &version,
        ShaderKind::Fragment,
        &frag_layout,
        Some(&vert_io_frag),
        None,
        &frag_entry,
    );

    Ok(PreProcOutput {
        vert_glsl,
        frag_glsl,
    })
}

fn build_glsl(
    version: &str,
    kind: ShaderKind,
    layout: &[&str],
    io_in: Option<&[&str]>,
    io_out: Option<&[&str]>,
    entry: &[&str],
) -> String {
    let mut glsl = String::new();

    glsl.push_str(&format!("#version {}", version));
    glsl.push('\n');
    glsl.push_str(&format!("#pragma shader_stage({})", kind));

    if let Some(layout_in) = io_in {
        for line in layout_in {
            let line = line.replace("OUT_IN", "in");
            glsl.push('\n');
            glsl.push_str(&line);
        }
    }

    for line in layout {
        glsl.push('\n');
        glsl.push_str(line);
    }

    if let Some(layout_out) = io_out {
        for line in layout_out {
            let line = line.replace("OUT_IN", "out");
            glsl.push('\n');
            glsl.push_str(&line);
        }
    }

    for line in entry {
        glsl.push('\n');
        glsl.push_str(line);
    }

    glsl
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

fn compile_and_write_to_stream(
    compiler: &shaderc::Compiler,
    options: &shaderc::CompileOptions,
    file: &str,
    kind: ShaderKind,
    source: &str,
    output: &mut (impl Write + Seek),
) -> RisResult<usize> {
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

    let extension = match kind {
        ShaderKind::Vertex => OUT_EXT[0],
        ShaderKind::Fragment => OUT_EXT[1],
    };

    let file = format!("{}.{}",file_stem, extension);

    let artifact = ris_error::unroll!(
        compiler.compile_into_spirv(
            source,
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

