#version 110

uniform sampler2D texture;
uniform vec4 color_mult;
varying vec2 v_tex_coords;

void main() {
    vec4 t = texture2D(texture, v_tex_coords);
    gl_FragColor = vec4(t.r, t.g, t.b, t.a) * color_mult;
}
