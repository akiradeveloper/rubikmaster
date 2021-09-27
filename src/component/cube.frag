#version 300 es

precision highp float;

in vec4 v_out_color;
out vec4 frag_color;

void main() {
    frag_color = v_out_color;
}
