#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec3 normal;

out vec4 out_Color;

//uniform mat4 r_transformation;
//uniform mat4 r_view;
//uniform mat4 r_projection;
//uniform mat4 r_vp;

layout (std140) uniform Matricies {
    mat4 view;
    mat4 projection;
    mat4 trans;
} matrix;

void main() {
    gl_Position = matrix.projection * matrix.view * matrix.trans * vec4(position, 1.0);
    out_Color = vec4(normal.xyz, uv.x);
}