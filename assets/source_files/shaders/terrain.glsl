#ris_glsl 450 vertex fragment

#include util/util.glsl

#define PLANET_RADIUS 3.0

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
    vec3 cube_vertex = vec3(in_vertex / PLANET_RADIUS, 1.0);
    float x = cube_vertex.x;
    float y = cube_vertex.y;
    float z = cube_vertex.z;
    float x2 = x * x;
    float y2 = y * y;
    float z2 = z * z;
    float sx = x * sqrt(1.0 - y2 / 2.0 - z2 / 2.0 + y2 * z2 / 3.0);
    float sy = y * sqrt(1.0 - x2 / 2.0 - z2 / 2.0 + x2 * z2 / 3.0);
    float sz = z * sqrt(1.0 - x2 / 2.0 - y2 / 2.0 + x2 * y2 / 3.0);
    vec3 sphere_vertex = vec3(sx, sy, sz);

    bool is_on_another_side = any(greaterThan(abs(cube_vertex),vec3(1.0)));

    if (is_on_another_side) {
        out_color = vec3(1.0, 0.0, 0.0);
    } else {
        out_color = vec3(1.0, 1.0, 1.0);
    }

    out_vertex = ubo.proj * ubo.view * ubo.model * vec4(sphere_vertex, 1.0);

    gl_Position = out_vertex;
}

#fragment
void main() {
    out_color = vec4(in_color, 1.0);
}
