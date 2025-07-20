#ris_glsl 450 vertex fragment

#include util/util.glsl

#vertex
layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 in_vertex;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;

#io vertex fragment
layout(location = 0) IN_OUT vec4 IN_OUT_vertex;
layout(location = 1) IN_OUT vec3 IN_OUT_color;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    out_vertex = ubo.proj * ubo.view * ubo.model * vec4(in_vertex, 1.0);
    out_color = vec3(1.0, 1.0, 1.0);

    gl_Position = out_vertex;
}

#fragment
void main() {
    out_color = vec4(in_color, 1.0);
}
