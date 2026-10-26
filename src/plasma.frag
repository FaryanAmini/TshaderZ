uniform float time;
uniform vec2 resolution;
layout (location = 0) out vec4 outColor;

void main() {
    vec2 uv = gl_FragCoord.xy / resolution;   // 0..1 across the central rect
    outColor = vec4(0.5 + 0.5 * sin(time), uv.x, uv.y, 1.0);
}
