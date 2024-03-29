use gl::{types::GLchar, GetUniformLocation, UniformMatrix4fv, UseProgram, FALSE, TRUE};
use glm::{perspective, translate, Mat4, Vec3, Vec4, identity, vec4, vec3};

use crate::{flatten::flatten, shader::ShaderProgram, quaternion::Quaternion};

use std::ffi::CString;

#[derive(Clone)]
pub struct Camera {
    pub(super) view: Mat4,
    pub(super) projection: Mat4,
    pub orientation: Quaternion,
    p: bool,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        let one = Vec4::new(1.0, 1.0, 1.0, 1.0);
        Self {
            view: identity(),
            projection: perspective(aspect, 45.0, 0.1, 100.0),
            orientation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            p: false,
        }
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        let translation = translation * -1.0;
        self.view = translate(&self.view, &translation);
        self
    }

    pub(super) fn send_to_shader(&mut self, shader: &ShaderProgram) {
        unsafe {
            shader.use_program();

            let view_name = CString::new("view").unwrap();
            let view_loc = GetUniformLocation(shader.0, view_name.as_ptr() as *const i8);
            UniformMatrix4fv(view_loc, 1, FALSE, flatten(self.view).as_ptr());

            let proj_name = CString::new("projection").unwrap();
            let projection_loc = GetUniformLocation(shader.0, proj_name.as_ptr() as *const i8);
            UniformMatrix4fv(projection_loc, 1, FALSE, flatten(self.projection).as_ptr());

            let orient_name = CString::new("orientation").unwrap();
            let orientation_loc = GetUniformLocation(shader.0, orient_name.as_ptr() as *const i8);
            UniformMatrix4fv(orientation_loc, 1, FALSE, flatten(self.orientation.to_matrix()).as_ptr());
        }

        if !self.p {
            println!("{:?}, v{:?}, p{:?}, p{:?}, pv{:?}, {:?}\n",
                vec3(0.5, 0.5, -1.0), flatten(self.view), flatten(self.projection),
                self.view * vec4(0.5, 0.5, -1.0, 1.0),
                self.projection * self.view * vec4(0.5, 0.5, -1.0, 1.0),
                self.projection);
            self.p = true;
        }
    }
}
