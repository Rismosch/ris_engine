#ris_glsl header

#define ONE 1.0

vec3 invert_color(vec3 c) {
    return (ONE - c) * 2;
}

vec2 viewport_coord(vec4 vertex) {
    vec3 ndc = vertex.xyz / vertex.w;
    vec2 result = ndc.xy * 0.5f + 0.5f;
    return result;
}

vec2 viewport_pixel_coord(vec4 vertex, vec2 viewport_size) {
    vec2 coord = viewport_coord(vertex);
    vec2 result = coord * viewport_size;
    return result;
}
