#ris_glsl 450 vertex fragment

#include util/util.glsl

#vertex
layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in ivec2 in_vertex;

#io vertex fragment
layout(location = 0) IN_OUT vec4 IN_OUT_vertex;
layout(location = 1) IN_OUT vec3 IN_OUT_color;

#fragment
layout(binding = 1) uniform sampler2D tex_sampler;

layout(location = 0) out vec4 out_color;

#vertex
void main() {
    out_vertex = ubo.proj * ubo.view * ubo.model * vec4(in_vertex, 0.0, 1.0);
    switch (gl_VertexIndex % 3) {
        case 0:
            out_color = vec3(1.0, 0.0, 0.0);
            break;
        case 1:
            out_color = vec3(0.0, 1.0, 0.0);
            break;
        case 2:
            out_color = vec3(0.0, 0.0, 1.0);
            break;
        default:
            out_color = vec3(1.0, 1.0, 1.0);
            break;
    }

    gl_Position = out_vertex;
}

#fragment
void main() {
    out_color = vec4(in_color, 1.0);
}
