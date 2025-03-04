#ris_glsl 450 vertex fragment

#vertex
layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_color;

#io vertex fragment
layout(location = 0) IN_OUT vec3 IN_OUT_color;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    gl_Position = ubo.proj * ubo.view * vec4(in_position, 1.0);
    out_color = in_color;
}

#fragment
void main() {
    out_color = vec4(in_color, 1.0);
}
