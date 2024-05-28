#version 330 core

uniform mat4 matrix;

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coords;
layout (location = 3) in int face_id;

out vec2 v_tex_coords;
out vec2 v_face_id;

void main() {
    gl_Position = matrix * vec4(position, 1.0);
    v_tex_coords = tex_coords;
    v_face_id = vec2(float(face_id), 0.0);
}