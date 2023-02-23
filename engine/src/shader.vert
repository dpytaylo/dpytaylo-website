#version 300 es

precision mediump float;

uniform mat4 u_mvp;

in vec3 a_position;
in vec2 a_tex_coord;

out vec2 v_tex_coord;

void main() {
    gl_Position = u_mvp * vec4(a_position, 1.0);
    v_tex_coord = a_tex_coord;
}