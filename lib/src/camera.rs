use gl::{types::GLchar, GetUniformLocation, UniformMatrix4fv, FALSE};
use glm::{ext::perspective, ext::translate, Mat4, Matrix4, Vec3, Vector3, Vector4};

use crate::{glm::flatten, shader::ShaderProgram};

#[derive(Clone)]
pub struct Camera {
    pub(super) view: Mat4,
    pub(super) projection: Mat4,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        let one = Vector4::new(1.0, 1.0, 1.0, 1.0);
        Self {
            view: Matrix4 {
                c0: one,
                c1: one,
                c2: one,
                c3: one,
            },
            projection: perspective(45.0, aspect, 0.1, 100.0),
        }
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        let translation = Vector3::new(-translation.x, -translation.y, -translation.z);
        translate(&mut self.view, translation);
        self
    }

    pub(super) fn send_to_shader(&self, shader: &ShaderProgram) {
        unsafe {
            let view_loc = GetUniformLocation(shader.0, "view".as_ptr() as *const i8);
            UniformMatrix4fv(view_loc, 1, FALSE, flatten(self.view).as_ptr());
            let projection_loc = GetUniformLocation(shader.0, "projection".as_ptr() as *const i8);
            UniformMatrix4fv(projection_loc, 1, FALSE, flatten(self.projection).as_ptr());
        }
    }
}
