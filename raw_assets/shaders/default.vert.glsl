#version 460
#pragma shader_stage(vertex)

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
    gl_Position = ubo.view_proj * vec4(position, 1.0);

    vec3 color_inv = vec3(1 - color.x, 1 - color.y, 1 - color.z);
    fragColor = gl_Position.zzz;
}
