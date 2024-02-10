#version 460 core

layout (location = 0) in vec3 pos;

uniform mat4 view;
uniform mat4 projection;

void main() {
  gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
