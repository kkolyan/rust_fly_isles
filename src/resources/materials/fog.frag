#version 100
varying lowp vec4 color;
varying lowp vec2 uv;

uniform sampler2D Texture;
uniform lowp vec4 FogColor;

void main() {
    lowp vec4 tc = texture2D(Texture, uv);
    tc.rgb = tc.rgb * (1.0 - FogColor.a) + FogColor.rgb * FogColor.a;
    gl_FragColor = tc;
}