use std::sync::Arc;

use vulkano::device::Device;
use vulkano::shader::ShaderModule;

pub type Shaders = (Arc<ShaderModule>, Arc<ShaderModule>);

pub fn compile_shaders(device: &Arc<Device>) -> Result<Shaders, String> {
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

    let compiler = shaderc::Compiler::new().ok_or("failed to initialize shaderc compiler")?;
    let options = shaderc::CompileOptions::new().ok_or("failed to initialize shaderc options")?;

    let vertex_artifact = compiler
        .compile_into_spirv(
            vertex_source,
            shaderc::ShaderKind::Vertex,
            "standard.vert",
            "main",
            Some(&options),
        )
        .map_err(|e| format!("failed to compile vertex shader: {}", e))?;
    let vertex_words: &[u32] = vertex_artifact.as_binary();
    let vertex_shader = unsafe { ShaderModule::from_words(device.clone(), vertex_words) }
        .map_err(|e| format!("failed to load vertex shader module: {}", e))?;

    let fragment_artifact = compiler
        .compile_into_spirv(
            fragment_source,
            shaderc::ShaderKind::Fragment,
            "standard.vert",
            "main",
            Some(&options),
        )
        .map_err(|e| format!("failed to compile fragment shader: {}", e))?;
    let fragment_words: &[u32] = fragment_artifact.as_binary();
    let fragment_shader = unsafe { ShaderModule::from_words(device.clone(), fragment_words) }
        .map_err(|e| format!("failed to lad fragment shader module: {}", e))?;

    Ok((vertex_shader, fragment_shader))
}
