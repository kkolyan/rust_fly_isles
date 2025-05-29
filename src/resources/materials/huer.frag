#version 100
precision lowp float;
varying lowp vec4 color;
varying lowp vec2 uv;

uniform sampler2D Texture;
uniform lowp vec4 SourceColor;
uniform lowp vec4 TargetColor;


vec3 rgb2hsv(vec3 c)
{
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    lowp vec4 tc = texture2D(Texture, uv);
    vec3 hsv_orig = rgb2hsv(tc.rgb);
    float f = distance(normalize(SourceColor.rgb), normalize(tc.rgb));
    tc.rgb = mix(TargetColor.rgb, tc.rgb, clamp(pow(f, 0.5), 0.0, 1.0));
    vec3 hsv = rgb2hsv(tc.rgb);

//    hsv[1] = (hsv_orig[1] + hsv[1]) * 0.5;
//    hsv[1] = hsv_orig[1] * hsv[1];
    hsv[1] = hsv_orig[1];

//    hsv[2] = (hsv_orig[2] + hsv[2]) * 0.5;
    hsv[2] = hsv_orig[2] * hsv[2];
//    hsv[2] = hsv_orig[2];

    tc.rgb = hsv2rgb(hsv);
//    tc.rgb = normalize(tc.rgb);
    gl_FragColor = tc;
}
