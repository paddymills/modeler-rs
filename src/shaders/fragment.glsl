#version 140

in vec3 v_normal;
out vec4 f_color;

const vec3 LIGHT_DIRECTION = vec3(-0.2, 0.4, 0.1);

void main() {
    float lum = max(dot(normalize(v_normal), normalize(LIGHT_DIRECTION)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * vec3(0.94, 0.68, 0.28);
    f_color = vec4(color, 1.0);
}