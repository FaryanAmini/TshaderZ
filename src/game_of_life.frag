uniform float time;
uniform float scale;
uniform vec4 viewport;
layout (location = 0) out vec4 outColor;

float hash(vec2 p) {
  return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
}

// procedural initial state: random 64x64 grid of cells (replaces u_texture for now)
float cell(vec2 uv) {
  return step(0.5, hash(floor(uv * 64.0)));
}

void main() {
  vec2 texel = 1.0 / vec2(64.0);  // one grid step in cell space

  // rect-local coords, then zoom around the center (uses viewport -> not stripped)
  vec2 uv = (gl_FragCoord.xy - viewport.xy) / viewport.zw;
  uv = (uv - 0.5) * scale + 0.5;

  // apply time: slowly rotate the sampling coordinates around the center
  float angle = time * 0.1;
  vec2 rotCenter = uv - 0.5;
  float ca = cos(angle);
  float sa = sin(angle);
  uv = vec2(rotCenter.x * ca - rotCenter.y * sa,
            rotCenter.x * sa + rotCenter.y * ca) + 0.5;

  // wrap to keep sampling inside the grid (toroidal Game of Life space)
  uv = fract(uv);

  // count live neighbors 3x3 kernel
  int liveNeighbors = 0;
  for (int y = -1; y <= 1; y++){
      for (int x = -1; x <= 1; x++){
          if (x == 0 && y == 0) continue;
          vec2 offset = vec2(float(x), float(y)) * texel;
          float neighbor = cell(fract(uv + offset));
          liveNeighbors += int(neighbor > 0.5);
      }
  }

  // Current cell state
  float self = cell(uv);
  bool isAlive = self > 0.5;

  // Apply Conway's Game of Life rules
  float nextState = 0.0;
  if (isAlive) {
      if (liveNeighbors == 2 || liveNeighbors == 3) {
          nextState = 1.0; // Stays alive
      }
  } else {
      if (liveNeighbors == 3) {
          nextState = 1.0; // Reproduction
      }
  }

  outColor = vec4(vec3(nextState), 1.0);
}
