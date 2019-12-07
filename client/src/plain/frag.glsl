#version 330 core
in vec4 out_Color;
out vec4 Color;

uniform vec3 color_r;

layout (std140) uniform Matricies {
    mat4 view;
    mat4 projection;
    mat4 trans;
} matrix;

void main(){
    Color = vec4(out_Color.x, out_Color.y, out_Color.z, 1.0f);
}