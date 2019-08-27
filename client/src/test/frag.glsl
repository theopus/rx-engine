#version 330 core
in vec3 out_Color;
out vec4 Color;

uniform vec3 color_r;

void main(){
    Color = vec4(color_r.x, color_r.y, color_r.z, 1.0f);
}