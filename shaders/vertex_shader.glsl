#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model_pos_mat;

void main() {
  mat4 model_view = view * model_pos_mat;
  v_normal = transpose(inverse(mat3(model_view))) * normal;
  gl_Position = projection * model_view * vec4(position, 1.0);

  v_position = gl_Position.xyz / gl_Position.w;
}