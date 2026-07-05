#version 320 es
precision highp float;
precision highp int;

uniform bool discard_transparent;

in vec4 vColor;

out vec4 color;

void main() {
    if (discard_transparent && vColor.a < 0.01) {
        discard;
    }
    color = vec4(vColor);
}
