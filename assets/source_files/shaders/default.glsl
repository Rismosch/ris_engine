#ris_glsl 450 vertex fragment

#include util/util.glsl

#vertex
layout(push_constant) uniform PushConstants {
    mat4 model;
} pc;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 in_vertex;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;

#io vertex fragment
layout(location = 0) IN_OUT vec4 IN_OUT_vertex;
layout(location = 1) IN_OUT vec3 IN_OUT_normal;
layout(location = 2) IN_OUT vec2 IN_OUT_uv;

#fragment
layout(binding = 1) uniform sampler2D tex_sampler;

layout(location = 0) out vec4 out_color;

#vertex
void main() {
    out_vertex = ubo.proj * ubo.view * pc.model * vec4(in_vertex, 1.0);
    out_normal = mat3(transpose(inverse(pc.model))) * in_normal;
    out_uv = in_uv;

    gl_Position = out_vertex;
}

#fragment
void main() {
    vec3 light_direction = vec3(1.0, -1.0, 1.0);
    vec3 light_color = vec3(0.981, 0.912, 0.788);
    vec3 ambient_color = vec3(0.624, 0.321, 0.096);
    vec3 object_color = vec3(1.0);

    float diff = max(dot(in_normal, light_direction), 0.0);
    vec3 diffuse = diff * light_color;

    //out_color = texture(tex_sampler, in_uv);

    vec3 result = (ambient_color + diffuse) * object_color;
    out_color = vec4(result, 1.0);
}
