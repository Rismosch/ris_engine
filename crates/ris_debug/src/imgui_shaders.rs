use std::sync::Arc;

use vulkano::device::Device;
use vulkano::shader::ShaderModule;

pub fn vertex_shader(device: &Arc<Device>) -> Result<Arc<ShaderModule>, String> {
    let source = "
        #version 450

        layout(push_constant) uniform VertPC {
            mat4 matrix;
        };

        layout(location = 0) in vec2 pos;
        layout(location = 1) in vec2 uv;
        layout(location = 2) in uint col;

        layout(location = 0) out vec2 f_uv;
        layout(location = 1) out vec4 f_color;

        // Built-in:
        // vec4 gl_Position

        void main() {
            f_uv = uv;
            f_color = unpackUnorm4x8(col);
            gl_Position = matrix * vec4(pos.xy, 0, 1);
        }
    ";

    compile(
        device,
        source,
        shaderc::ShaderKind::Vertex,
        "imgui.vert"
    )
}

pub fn fragment_shader(device: &Arc<Device>) -> Result<Arc<ShaderModule>, String> {
    let source = "
        #version 450

        layout(binding = 0) uniform sampler2D tex;

        layout(location = 0) in vec2 f_uv;
        layout(location = 1) in vec4 f_color;

        layout(location = 0) out vec4 Target0;

        void main() {
            Target0 = f_color * texture(tex, f_uv.st);
        }
    ";

    compile(
        device,
        source,
        shaderc::ShaderKind::Fragment,
        "imgui.frag"
    )
}

fn compile(
    device: &Arc<Device>,
    source: &str,
    kind: shaderc::ShaderKind,
    name: &str) -> Result<Arc<ShaderModule>, String> {
    let compiler = shaderc::Compiler::new().ok_or("failed to initialize shader compiler")?;
    let options = shaderc::CompileOptions::new().ok_or("failed to initialize shaderc options")?;

    let artifact = compiler
        .compile_into_spirv(
            source,
            kind,
            name,
            "main",
            Some(&options),
        )
        .map_err(|e| format!("failed to compile shader \"{}\": {}", name, e))?;

    let words: &[u32] = artifact.as_binary();
    let shader = unsafe { ShaderModule::from_words(device.clone(), words)}
        .map_err(|e| format!("failed to load shader module \"{}\": {}", name, e))?;

    Ok(shader)
}
