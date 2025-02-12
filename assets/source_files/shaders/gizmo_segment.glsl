#ris_glsl 450 vertex geometry fragment

#include util/util.glsl
#define QUAD_THICKNESS 0.002
#define LINE_THICKNESS 0.0015

#vertex
layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_color;

#io vertex geometry
layout(location = 0) IN_OUT vec3 IN_OUT_color;

#geometry
layout(lines) in;
layout (triangle_strip, max_vertices = 4) out;

#io geometry fragment
layout(location = 0) IN_OUT vec3 IN_OUT_color;
layout(location = 1) IN_OUT vec4 IN_OUT_vertex;
layout(location = 2) IN_OUT vec4 line_start;
layout(location = 3) IN_OUT vec4 line_end;

#fragment
layout(location = 0) out vec4 out_color;

#geometry
// visualization: https://www.desmos.com/calculator/dfa2rsv2la
vec2 clamp_to_viewport(vec2 v, float m, float c) {
    // clamp to x
    v.y = choose(
        v.x < -1,
        c - m,
        choose(
            v.x > 1,
            c + m,
            v.y
        )
    );
    v.x = clamp(v.x, -1, 1);

    // clamp to y
    v.x = choose(
        v.y < -1,
        (-1 - c) / m,
        choose(
            v.y > 1,
            (1 - c) / m,
            v.x
        )
    );
    v.y = clamp(v.y, -1, 1);

    return v;
}

#vertex
void main() {
    gl_Position = ubo.proj * ubo.view * vec4(in_position, 1.0);
    out_color = in_color;
}

#geometry
void main() {
    vec4 v0 = gl_in[0].gl_Position;
    vec4 v1 = gl_in[1].gl_Position;
    vec3 c0 = in_color[0];
    vec3 c1 = in_color[1];

    // make sure v0 is alwas nearer to the near plane than v1
    // this ensures the clipping below works in either case
    if (v0.z > v1.z) {
        vec4 vt = v0;
        v0 = v1;
        v1 = vt;
    }

    // if both vertices are behind the near plane, clip them
    if (v0.z < -v0.w && v1.z < -v1.w)
    {
        return;
    }

    // if either vertex is behind the near plane, clamp them
    if (v0.z < -v0.w || v1.z < -v1.w) {
        float t = (v0.w + v0.z) / (v0.w + v0.z - v1.w - v1.z);
        if (v0.z < -v0.w) {
            v0 = v0 + t * (v1 - v0);
        } else {
            v1 = v1 + t * (v0 - v1);
        }
    }

    // ndc
    vec4 ndc0 = vec4(v0.xyz / v0.w, 1);
    vec4 ndc1 = vec4(v1.xyz / v1.w, 1);

    // clamp ndc0 and ndc1 into the viewport
    if (abs(ndc0.x - ndc1.x) < 0.000001) {
        // segment is vertical
        ndc0.y = clamp(ndc0.y, -1, 1);
        ndc1.y = clamp(ndc1.y, -1, 1);
    } else {
        float m = (ndc0.y - ndc1.y) / (ndc0.x - ndc1.x);
        float c = ndc0.y - m * ndc0.x;
        ndc0.xy = clamp_to_viewport(ndc0.xy, m, c);
        ndc1.xy = clamp_to_viewport(ndc1.xy, m, c);
    }
    
    // calculate quad offset
    vec2 dir = ndc1.xy - ndc0.xy;
    vec2 offset_dir = normalize(vec2(-dir.y, dir.x));
    vec4 offset = QUAD_THICKNESS * vec4(offset_dir, 0, 0);

    // emit vertices
    out_color = c0;
    out_vertex = ndc0 + offset;
    line_start = ndc0;
    line_end = ndc1;
    gl_Position = out_vertex;
    EmitVertex();

    out_color = c0;
    out_vertex = ndc0 - offset;
    line_start = ndc0;
    line_end = ndc1;
    gl_Position = out_vertex;
    EmitVertex();

    out_color = c1;
    out_vertex = ndc1 + offset;
    line_start = ndc0;
    line_end = ndc1;
    gl_Position = out_vertex;
    EmitVertex();

    out_color = c1;
    out_vertex = ndc1 - offset;
    line_start = ndc0;
    line_end = ndc1;
    gl_Position = out_vertex;
    EmitVertex();

    EndPrimitive();
}

#fragment
void main() {
    vec2 uv = screen_pos(in_vertex);
    vec2 p1 = screen_pos(line_start);
    vec2 p2 = screen_pos(line_end);

    vec2 vec_b = p2 - p1;
    vec2 vec_c = uv - p1;
    float alpha = acos(dot(normalize(vec_c), normalize(vec_b)));
    float distance_to_line = length(vec_c) * sin(alpha);

    float line = 1 - distance_to_line / LINE_THICKNESS;

    out_color = vec4(in_color, line);
    //out_color = vec4(in_vertex.zzz, 1);
}
