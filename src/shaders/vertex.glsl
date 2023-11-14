#version 140

uniform mat4 persp_matrix;
uniform mat4 view_matrix;
uniform mat4 rotx_matrix;
uniform mat4 roty_matrix;
uniform mat4 rotz_matrix;

in vec3 position;
in vec3 normal;
out vec3 v_position;
out vec3 v_normal;

void main() {
    v_position = position;
    v_normal = normal;
    gl_Position = persp_matrix * view_matrix * rotx_matrix * roty_matrix * rotz_matrix * vec4(v_position * 0.005, 1.0);
}
