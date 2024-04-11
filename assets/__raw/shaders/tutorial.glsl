#ris_glsl 450 vertex fragment

#vertex
layout(location = 0) in vec2 in_position;
layout(location = 1) in vec3 in_color;

#io vertex fragment
layout(location = 0) IN_OUT vec3 frag_color;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    gl_Position = vec4(in_position, 0.0, 1.0);
    frag_color = in_color;
}

#fragment
void main() {
    out_color = vec4(frag_color, 1.0);
}
