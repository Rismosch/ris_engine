#ris_glsl 450 vertex geometry fragment

#include util/util.glsl

#define GLYPH_PIXEL_SIZE 16

#vertex
layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
    uint screen_width;
    uint screen_height;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in uint in_text_addr;
layout(location = 2) in uint in_text_len;

#io vertex geometry
layout(location = 0) IN_OUT uint IN_OUT_text_addr;
layout(location = 1) IN_OUT uint IN_OUT_text_len;
layout(location = 2) IN_OUT uint IN_OUT_screen_width;
layout(location = 3) IN_OUT uint IN_OUT_screen_height;

#geometry
layout(points) in;
layout (triangle_strip, max_vertices = 128) out;

#io geometry fragment
layout(location = 0) IN_OUT vec2 uv;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    gl_Position = ubo.proj * ubo.view * vec4(in_position, 1.0);
    out_text_addr = in_text_addr;
    out_text_len = in_text_len;
    out_screen_width = ubo.screen_width;
    out_screen_height = ubo.screen_height;
}

#geometry
void main() {
    vec4 v = gl_in[0].gl_Position;
    uint text_addr = in_text_addr[0];
    uint text_len = in_text_len[0];
    uint screen_width = in_screen_width[0];
    uint screen_height = in_screen_height[0];

    if (text_len == 0) {
        return;
    }

    float glyph_offset_x = 2.0 * GLYPH_PIXEL_SIZE / float(screen_width);
    float glyph_offset_y = 2.0 * GLYPH_PIXEL_SIZE / float(screen_height);

    vec4 ndc = vec4(v.xyz / v.w, 1);
    vec4 origin = vec4(
        ndc.x - glyph_offset_x * text_len * 0.5,
        ndc.y - glyph_offset_y * 0.5,
        0,
        1
    );

    for (uint i = 0; i < text_len; ++i) {
        vec4 v0 = vec4(i * glyph_offset_x, 0, 0, 0);
        vec4 v1 = vec4(i * glyph_offset_x, glyph_offset_y, 0, 0);
        vec4 v2 = vec4((i + 1) * glyph_offset_x, 0, 0, 0);
        vec4 v3 = vec4((i + 1) * glyph_offset_x, glyph_offset_y, 0, 0);

        uv = vec2(-1, -1);
        gl_Position = origin + v0;
        EmitVertex();

        uv = vec2(-1, 1);
        gl_Position = origin + v1;
        EmitVertex();

        uv = vec2(1, -1);
        gl_Position = origin + v2;
        EmitVertex();

        uv = vec2(1, 1);
        gl_Position = origin + v3;
        EmitVertex();

        EndPrimitive();
    }
}

#fragment
void main() {
    out_color = vec4(uv, 0, 1);
}
