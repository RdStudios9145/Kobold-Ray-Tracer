#version 460 core

layout (location = 0) in vec3 pos;

uniform mat4 cam_view;
uniform mat4 cam_projection;
uniform mat4 cam_orientation;

uniform vec3 obj_position;
uniform vec3 obj_scale;
uniform mat4 obj_rotation;

// uniform mat4 obj_mat;

void main() {
  mat4 oscale = mat4(
    obj_scale.x, 0, 0, 0,
    0, obj_scale.y, 0, 0,
    0, 0, obj_scale.y, 0,
    0, 0, 0, 1
  );

  mat4 opos = mat4(
    1, 0, 0, 0,
    0, 1, 0, 0,
    0, 0, 1, 0,
    obj_position.x, obj_position.y, obj_position.z, 1
  ); 

  // 1 2 3 4    1 5 9 4
  // 5 6 7 8    2 6 1 5
  // 9 1 2 3    3 7 2 6
  // 4 5 6 7    4 8 3 7

  mat4 obj_mat = opos * obj_rotation * oscale;

  // vec4 obj_pos = vec4(obj_position, 1.0) * obj_rotation * vec4(obj_scale, 1.0) * vec4(pos, 1.0);
  // vec4 obj_pos = vec4(pos, 1.0);
  // vec4 obj_pos = obj_mat * vec4(pos, 1.0);
  // gl_Position = cam_projection * cam_orientation * cam_view * obj_pos;
  gl_Position = cam_projection * cam_orientation * cam_view * obj_mat * vec4(pos, 1.0);
  // gl_Position = cam_projection * cam_view * obj_mat * vec4(pos, 1.0);
}
