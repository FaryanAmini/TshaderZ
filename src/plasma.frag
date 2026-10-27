uniform float time;
uniform float scale;
uniform vec4 viewport;
layout (location = 0) out vec4 outColor;

void main() {
    vec2 uv = (gl_FragCoord.xy - viewport.xy) / viewport.zw;   // true 0..1 across the central rect
    uv = (uv - 0.5) * scale + 0.5;            // zoom the pattern by scaling around the center
    // smooth sine bands:
    float r = 0.5 + 0.5 * sin(time);
    float g = 0.5 + 0.5 * sin(uv.x * 6.28318);
    float b = 0.5 + 0.5 * sin(uv.y * 6.28318);
    outColor = vec4(r, g, b, 1.0);

}
