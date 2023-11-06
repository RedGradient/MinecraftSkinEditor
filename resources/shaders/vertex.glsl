#version 330 core

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;

out vec4 vColor;

void main() {
    gl_Position = perspective_matrix * view_matrix * model_matrix * vec4(position, 1.0);
    vColor = color;
}