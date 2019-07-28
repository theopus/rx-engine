#version 330 core
layout (location = 0) in vec3 Position;

out vec3 out_Color;

uniform mat4 m;

void main() {
    gl_Position = m * vec4(Position, 1.0);
    out_Color = gl_Position.xyz;
}