#version 330 core
#extension GL_ARB_separate_shader_objects : enable
layout (location = 0) in vec2 varying_uv;

layout (location = 0) out vec4 target;
void main() {
    target = vec4(varying_uv.x, varying_uv.y, 1, 1);
}