#ris_glsl 450 vertex fragment

#include util/util.glsl

#vertex
layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_color;
layout(location = 2) in vec2 in_uv;

#io vertex fragment
layout(location = 0) IN_OUT vec4 IN_OUT_position;
layout(location = 1) IN_OUT vec3 IN_OUT_color;
layout(location = 2) IN_OUT vec2 IN_OUT_uv;

#fragment
layout(binding = 1) uniform sampler2D tex_sampler;

layout(location = 0) out vec4 out_color;

#vertex
void main() {
    out_position = ubo.proj * ubo.view * ubo.model * vec4(in_position, 1.0);
    out_color = in_color;
    out_uv = in_uv;

    gl_Position = out_position;
}

#fragment
void main() {
    out_color = texture(tex_sampler, in_uv);
    //out_color = vec4(in_uv, 0.0, 1.0);
}
