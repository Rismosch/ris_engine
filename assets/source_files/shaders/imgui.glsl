#ris_glsl 460 vertex fragment

#vertex
layout(location = 0) in vec2 v_position;
layout(location = 1) in vec2 v_uv;
layout(location = 2) in vec4 v_color;

layout(push_constant) uniform Matrices {
    mat4 ortho;
} matrices;

#io vertex fragment
layout(location = 0) IN_OUT vec4 f_color;
layout(location = 1) IN_OUT vec2 f_uv;

#fragment
layout(binding = 0) uniform sampler2D fonts_sampler;

layout(location = 0) out vec4 out_color;

#vertex
void main() {
    f_color = v_color;
    f_uv = v_uv;
    gl_Position = matrices.ortho * vec4(v_position.xy, 0, 1);
}

#fragment
void main() {
    out_color = f_color * texture(fonts_sampler, f_uv);
}
