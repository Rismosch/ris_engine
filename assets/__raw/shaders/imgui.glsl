#glsl_version 460

#layout vertex
layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;
layout(location = 2) in uint col;

layout(push_constant) uniform VertPC {
    mat4 matrix;
};

#layout io vertex fragment
layout(location = 0) OUT_IN vec2 f_uv;
layout(location = 1) OUT_IN vec4 f_color;

#layout fragment
layout(binding = 0) uniform sampler2D tex;

layout(location = 0) out vec4 Target0;

#entry vertex
void main() {
    f_uv = uv;
    f_color = unpackUnorm4x8(col);
    gl_Position = matrix * vec4(pos.xy, 0, 1);
}

#entry fragment
void main() {
    Target0 = f_color * texture(tex, f_uv.st);
}
