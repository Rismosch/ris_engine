#ris_glsl header

#define ONE 1.0

vec3 test(vec3 c) {
    return (ONE - c) * 2;
}

// given a clip space position `clip_pos`, this function returns the resulting position of this position on the screen. prefere to use this function in the fragment shader, as perspective interpolation messes this up.
vec2 screen_pos(vec4 clip_pos) {
    vec3 ndc = clip_pos.xyz / clip_pos.w;
    vec2 result = ndc.xy * 0.5f + 0.5f;
    return result;
}
