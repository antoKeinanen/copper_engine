#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 position_matrix;

void main() {
  mat4 model_view = view * position_matrix;
  v_normal = transpose(inverse(mat3(model_view))) * normal;
  gl_Position = perspective * model_view * vec4(position, 1.0);
  v_position = gl_Position.xyz / gl_Position.w;
}