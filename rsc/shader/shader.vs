#version 330

layout(location = 0) in vec3 iPosition;
layout(location = 1) in vec3 iNormal;
layout(location = 2) in vec2 iTexCoords;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;
uniform float uAlpha;

out float Alpha;
out vec3 FragPosition;
out vec3 Normal;
out vec2 TexCoords;

void main()
{
    Alpha = uAlpha;
    FragPosition = vec3(uModel * vec4(iPosition, 1.0));
    Normal = mat3(transpose(inverse(uModel))) * iNormal;
    TexCoords = iTexCoords;
    gl_Position = uProjection * uView * vec4(FragPosition, 1.0);
}