#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec3 normal;
//instance
layout (location = 3) in float material_id;
layout (location = 4) in mat4 transformation;
layout (location = 8) in mat4 mvp;

layout (std140) uniform Matricies matrix;

struct Matricies {
    mat4 view;
    mat4 projection;
    mat4 vp;
};

void main() {
    gl_Position = mvp * vec4(position.xyz, 1.0);
}