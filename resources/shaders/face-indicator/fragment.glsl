#version 330 core

in vec2 v_tex_coords;
in vec2 v_face_id;

out vec4 color;

uniform sampler2D front_tex;
uniform sampler2D back_tex;
uniform sampler2D right_tex;
uniform sampler2D left_tex;
uniform sampler2D top_tex;
uniform sampler2D bottom_tex;

void main() {
    int id = int(v_face_id[0]);
    if (id == 0) {
        color = texture(front_tex, v_tex_coords);
    } else if (id == 1) {
        color = texture(back_tex, v_tex_coords);
    } else if (id == 2) {
        color = texture(top_tex, v_tex_coords);
    } else if (id == 3) {
        color = texture(bottom_tex, v_tex_coords);
    } else if (id == 4) {
        color = texture(right_tex, v_tex_coords);
    } else if (id == 5) {
        color = texture(left_tex, v_tex_coords);
    }
}