#version 460 core

layout (location = 0) in vec3 pos;

uniform mat4 view;
uniform mat4 projection;

void main() {
  // gl_Position = view * vec4(pos, 1.0);
  gl_Position = projection * view * vec4(pos, 1.0);
  // gl_Position = vec4(view[0].xyz, 1.0) * vec4(pos, 1.0);
}
