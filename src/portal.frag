uniform float time;
uniform float scale;
uniform vec4 viewport;

#define PI 3.14
#define uTime (time+4.)/15.


float sintime(float lowerBound, float upperBound, float offset) {
    return mix(lowerBound, upperBound, (sin(uTime*PI*2. + offset)*.5+.5));
}

mat2 g(float a) { // creates a 2d rotation matrix
    return mat2(cos(a), -sin(a), sin(a), cos(a));
}

// Shadertoy compat: iResolution is injected by their host; we map it to our viewport rect
#define iResolution viewport.zw

void mainImage( out vec4 fragColor, in vec2 fragCoord ) {
    vec2 uv = ( fragCoord - .5*iResolution )/(iResolution.y*scale);

    vec3 finalColor;
    vec2 uv0 = uv;

    uv = fract(abs(uv) * (6.4 + sintime(-3., 1., 0.)))-.5;

    for(int c = 0; c < 3; c++) {
        float dst = 1e6;

        for(float j = -1.; j <= 1.; j++) {
            for (float i = -1.; i <= 1.; i++) {
                vec2 nuv = uv + vec2(i, j);

                float l = length(nuv);
                float ma = l + sintime(-1.,1.,l);
                float ic = l * length(uv0) * sintime(-1.,1.,0.)*14. + uTime*ma*.05 + float(c)*0.5*sintime(.1, 1., PI);

                nuv = nuv * ma*g(ic) + vec2(sintime(-.4,.8,.0), sintime(-.4,.8,PI/2.)); // apply the magic
                dst = min(dst, length(nuv));
            }
        }

        uv += dst * sintime(0., .08, 0.);

        finalColor[c] = .15/dst;
    }

    fragColor = vec4(finalColor,1.0);
}

out vec4 fragColor;
void main() {
    // rect-local pixel coords (gl_FragCoord is window-based, like the old uv anchor bug)
    mainImage(fragColor, gl_FragCoord.xy - viewport.xy);
}
