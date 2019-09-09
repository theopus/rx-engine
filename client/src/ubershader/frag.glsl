#version 330 core
#extension GL_ARB_separate_shader_objects : enable
//layout (location = 1) in vec2 uv;

layout (location = 0) out vec4 target;
void main() {
    target = vec4(1, 1, 1, 1);
}