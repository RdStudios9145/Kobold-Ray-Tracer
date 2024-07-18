#version 460 core

layout (location = 0) in vec3 pos;

uniform mat4 cam_view;
uniform mat4 cam_projection;
uniform mat4 cam_orientation;

// uniform vec3 obj_position;
// uniform vec3 obj_scale;
// uniform mat4 obj_rotation;

uniform mat4 obj_mat;

void main() {
  // vec4 obj_pos = vec4(obj_position, 1.0) * obj_rotation * vec4(obj_scale, 1.0) * vec4(pos, 1.0);
  // vec4 obj_pos = vec4(pos, 1.0);
  vec4 obj_pos = obj_mat * vec4(pos, 1.0);
  gl_Position = cam_projection * cam_orientation * cam_view * obj_pos;
}
