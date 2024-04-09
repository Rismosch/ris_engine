#ris_glsl 450 vertex fragment

#vertex
vec2 positions[3] = vec2[](
    vec2(0.0, 0.5),
    vec2(-0.5, -0.5),
    vec2(0.5, -0.5)
);

vec3 colors[3] = vec3[](
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0)
);

#io vertex fragment
layout(location = 0) IN_OUT vec3 frag_color;

#fragment
layout(location = 0) out vec4 out_color;

#vertex
void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    frag_color = colors[gl_VertexIndex];
}

#fragment
void main() {
    out_color = vec4(frag_color, 1.0);
}
