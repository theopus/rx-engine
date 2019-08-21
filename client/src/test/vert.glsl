#version 330 core
layout (location = 0) in vec3 Position;

out vec3 out_Color;

uniform mat4 r_transformation;
uniform mat4 r_view;
uniform mat4 r_projection;
uniform mat4 r_vp;

void main() {
    gl_Position = r_vp * r_transformation *  vec4(Position, 1.0);
    out_Color = gl_Position.xyz;
}