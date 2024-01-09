#version 460
#pragma shader_stage(vertex)

layout(push_constant) uniform VertPC {
    mat4 matrix;
};

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;
layout(location = 2) in uint col;

layout(location = 0) out vec2 f_uv;
layout(location = 1) out vec4 f_color;

void main() {
    f_uv = uv;
    f_color = unpackUnorm4x8(col);
    gl_Position = matrix * vec4(pos.xy, 0, 1);
}
