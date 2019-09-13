#version 330 core
in vec3 out_Color;
out vec4 Color;

uniform vec3 color_r;

void main(){
    Color = vec4(out_Color.x, out_Color.y, out_Color.z, 1.0f);
}