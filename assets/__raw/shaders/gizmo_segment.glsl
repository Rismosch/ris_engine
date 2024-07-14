#ris_glsl 450 vertex geometry fragment

#vertex
//layout(set = 0, binding = 0) uniform UniformBufferObject {
//    mat4 model;
//    mat4 view;
//    mat4 proj;
//} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_color;

#io vertex geometry
layout(location = 0) IN_OUT vec3 IN_OUT_color;

#geometry
layout(lines) in;
layout (triangle_strip, max_vertices = 4) out;

#io geometry fragment
layout(location = 0) IN_OUT vec3 IN_OUT_color;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    //gl_Position = ubo.proj * ubo.view * ubo.model * vec4(in_position, 1.0);
    gl_Position = vec4(in_position, 1.0);
    out_color = in_color;
}

#geometry
void main() {
    vec4 v0 = gl_in[0].gl_Position;
    vec4 v1 = gl_in[1].gl_Position;
    vec3 c0 = in_color[0];
    vec3 c1 = in_color[1];
    vec4 offset = vec4(0, 0.1, 0, 0);

    gl_Position = v0 + offset;
    out_color = c0;
    EmitVertex();

    gl_Position = v0 - offset;
    out_color = c0;
    EmitVertex();

    gl_Position = v1 + offset;
    out_color = c1;
    EmitVertex();

    gl_Position = v1 - offset;
    out_color = c1;
    EmitVertex();

    EndPrimitive();
}

#fragment
void main() {
    out_color = vec4(in_color, 1.0);
}
