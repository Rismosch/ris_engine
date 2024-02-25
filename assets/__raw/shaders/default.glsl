#ris_glsl 460 vertex fragment

#include util/util.glsl

#vertex
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
    mat4 proj_view;
} ubo;

#io vertex fragment
layout(location = 0) IN_OUT vec3 f_color;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    gl_Position = ubo.proj_view * vec4(position, ONE);

    f_color = color;
    //f_color = vec3(viewport_coord(gl_Position), 0);
}

#fragment
void main() {
    out_color = vec4(f_color, ONE);
}
