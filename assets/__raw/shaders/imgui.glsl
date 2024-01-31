#ris_glsl 460 vertex fragment

#vertex
layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;
layout(location = 2) in uint col;

layout(push_constant) uniform VertPC {
    mat4 matrix;
};

#io vertex fragment
layout(location = 0) IN_OUT vec2 f_uv;
layout(location = 1) IN_OUT vec4 f_color;

#fragment
layout(binding = 0) uniform sampler2D tex;

layout(location = 0) out vec4 Target0;

#vertex
void main() {
    f_uv = uv;
    f_color = unpackUnorm4x8(col);
    gl_Position = (matrix * vec4(pos.xy, 0, 1));
}

#fragment
void main() {
    Target0 = f_color * texture(tex, f_uv.st);
}
