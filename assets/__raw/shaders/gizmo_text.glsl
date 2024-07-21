#ris_glsl 450 vertex geometry fragment

#include util/util.glsl

#vertex
layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
    vec2 resolution;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in uint in_text_addr;
layout(location = 2) in uint in_text_len;

#io vertex geometry
layout(location = 0) IN_OUT uint IN_OUT_text_addr;
layout(location = 1) IN_OUT uint IN_OUT_text_len;

#geometry
layout(points) in;
layout (triangle_strip, max_vertices = 256) out;

#io geometry fragment
layout(location = 0) IN_OUT vec2 uv;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    //gl_Position = ubo.proj * ubo.view * vec4(in_position, 1.0);
    gl_Position = vec4(0, 0, 0, 0);
    out_text_addr = in_text_addr;
    out_text_len = in_text_len;
}

#geometry
void main() {
    vec4 v = gl_in[0].gl_Position;
    uint text_addr = in_text_addr[0];
    uint text_len = in_text_len[0];

    uv = vec2(-0.5, -0.5);
    gl_Position = vec4(uv, 0, 1);
    EmitVertex();

    uv = vec2(-0.5, 0.5);
    gl_Position = vec4(uv, 0, 1);
    EmitVertex();

    uv = vec2(-0.5, -0.5);
    gl_Position = vec4(uv, 0, 1);
    EmitVertex();

    uv = vec2(0.5, -0.5);
    gl_Position = vec4(uv, 0, 1);
    EmitVertex();

    EndPrimitive();
}

#fragment
void main() {
    out_color = vec4(uv, 0, 1);
}
