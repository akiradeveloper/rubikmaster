#version 300 es

uniform mat4 u_rotation;
uniform mat4 u_model_view;
uniform mat4 u_projection;
in vec3 v_in_position;
in vec4 v_in_color;
out vec4 v_out_color;

void main() {
    gl_Position = u_projection * u_model_view * u_rotation * vec4(v_in_position, 1.0);
    v_out_color = v_in_color;
}