#ris_glsl header

// given a clip space position `clip_pos`, this function returns the resulting position of this position on the screen. prefere to use this function in the fragment shader, as perspective interpolation messes this up.
vec2 screen_pos(vec4 clip_pos) {
    vec3 ndc = clip_pos.xyz / clip_pos.w;
    vec2 result = ndc.xy * 0.5f + 0.5f;
    return result;
}

/// returns x if a = true, and y is a = false. useful for branchless programming
float choose(bool a, float x, float y) {
    return mix(y, x, a);
}
