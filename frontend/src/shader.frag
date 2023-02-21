#version 300 es

precision mediump float;

uniform sampler2D u_texture;

in vec2 v_tex_coord;

out vec4 output_color;

void main() {
    output_color = texture(u_texture, vec2(v_tex_coord.x, 1.0 - v_tex_coord.y));
    //output_color = vec4(vec2(v_tex_coord.x, 1.0 - v_tex_coord.y), 0.0, 1.0);
}