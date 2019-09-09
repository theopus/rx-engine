#version 330 core
//#extension GL_ARB_separate_shader_objects : enable
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec3 normal;
//instance
//layout (location = 3) in float material_id;
//layout (location = 4) in mat4 transformation;
//layout (location = 8) in mat4 mvp;

layout (std140) uniform Matricies {
    mat4 view;
    mat4 projection;
    mat4 vp;
} matrix;



void main() {
    gl_Position = vec4(position.xyz, 1.0);
}