use std::sync::Arc;

use vulkano::device::Device;
use vulkano::shader::ShaderModule;

use ris_util::ris_error::RisError;

pub type Shaders = (Arc<ShaderModule>, Arc<ShaderModule>);

pub fn compile_shaders(device: &Arc<Device>) -> Result<Shaders, RisError> {
    let vertex_source = "
        #version 460

        layout(set = 0, binding = 0) uniform UniformBufferObject {
            mat4 view;
            mat4 proj;
            mat4 view_proj;

            int debug_x;
            int debug_y;
        } ubo;

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec3 color;

        layout(location = 0) out vec3 fragColor;

        void main() {
            vec3 p = vec3(position.x, position.y, position.z);

            gl_Position = ubo.view_proj * vec4(p, 1.0);

            fragColor = color;
        }
    ";

    let fragment_source = "
        #version 460

        layout(location = 0) in vec3 fragColor;

        layout(location = 0) out vec4 f_color;

        void main() {
            f_color = vec4(fragColor, 1.0);
        }
    ";

    let compiler = ris_util::unroll_option!(
        shaderc::Compiler::new(),
        "failed to initialize shaderc compiler"
    )?;
    let options = ris_util::unroll_option!(
        shaderc::CompileOptions::new(),
        "failed to initialize shaderc options"
    )?;

    let vertex_artifact = ris_util::unroll!(
        compiler.compile_into_spirv(
            vertex_source,
            shaderc::ShaderKind::Vertex,
            "standard.vert",
            "main",
            Some(&options),
        ),
        "failed to compile vertex shader"
    )?;
    let vertex_words: &[u32] = vertex_artifact.as_binary();
    let vertex_shader = ris_util::unroll!(
        unsafe { ShaderModule::from_words(device.clone(), vertex_words) },
        "failed to load vertex shader module"
    )?;

    let fragment_artifact = ris_util::unroll!(
        compiler.compile_into_spirv(
            fragment_source,
            shaderc::ShaderKind::Fragment,
            "standard.vert",
            "main",
            Some(&options),
        ),
        "failed to compile fragment shader"
    )?;
    let fragment_words: &[u32] = fragment_artifact.as_binary();
    let fragment_shader = ris_util::unroll!(
        unsafe { ShaderModule::from_words(device.clone(), fragment_words) },
        "failed to lad fragment shader module"
    )?;

    Ok((vertex_shader, fragment_shader))
}
