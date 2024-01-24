#version 460
#pragma shader_stage(vertex)

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
    mat4 view_proj;
} ubo;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 frag_color;

void main() {
    //gl_Position = vec4(position, 1.0) * ubo.view_proj;
    //gl_Position = vec4(position, 1.0) * ubo.view * ubo.proj;
    //gl_Position = ubo.proj * ubo.view * vec4(position, 1.0);
    gl_Position = ubo.view_proj * vec4(position, 1.0);

    frag_color = color;
}

