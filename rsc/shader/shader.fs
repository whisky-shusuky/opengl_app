#version 140

in vec3 FragPosition;
out vec4 FragColor;

void main()
{
    // とりあえず色付けだけで何もしない
    FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}