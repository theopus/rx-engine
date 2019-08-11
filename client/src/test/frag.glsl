#version 330 core
in vec3 out_Color;
out vec4 Color;

void main(){
    Color = vec4(out_Color.x + 3, out_Color.y + 0.5f, out_Color.z + 0.52f, 1.0f);
}