#ris_glsl header

#define ONE 1.0

vec3 invert_color(vec3 c) {
    return (ONE - c) * 2;
}
