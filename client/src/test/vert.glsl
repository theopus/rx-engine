#version 330 core
layout (location = 0) in vec3 Position;

out vec3 out_Color;

uniform mat4 m;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 vp;

void main() {
    gl_Position = vp * m *  vec4(Position, 1.0);
    out_Color = gl_Position.xyz;
}