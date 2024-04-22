use gl::{GetUniformLocation, UniformMatrix4fv, FALSE};
use glm::{perspective, translate, Mat4, Vec3, identity, vec4, vec3};

use crate::{flatten::flatten, shader::ShaderProgram, quaternion::Quaternion};

use std::ffi::CString;

pub use crate::rotatable::Rotatable;

#[derive(Clone, Copy)]
pub struct Camera {
    pub(super) view: Mat4,
    pub(super) projection: Mat4,
    pub orientation: Quaternion,
    pub position: Vec3,
    p: bool,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            view: identity(),
            projection: perspective(aspect, 45.0, 0.1, 100.0),
            orientation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
            position: vec3(0., 0., 0.),
            p: true,
        }
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        self.position += translation;
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
            println!("{:?}, v{:?}, p{:?}, p{:?}, pv{:?}, rpv{:?}, {:?}\n",
                vec3(0.5, 0.5, -1.0), flatten(self.view), flatten(self.projection),
                self.view * vec4(0.5, 0.5, -1.0, 1.0),
                self.projection * self.view * vec4(0.5, 0.5, -1.0, 1.0),
                self.orientation.to_matrix() * self.projection * self.view * vec4(0.5, 0.5, -1.0, 1.0),
                self.projection);
            self.p = true;
        }
    }
}

impl Rotatable for Camera {
    fn set_orientation(&mut self, q: Quaternion) { self.orientation = q }
    fn get_orientation(&self) -> Quaternion { self.orientation }
    fn normalize(&mut self) { self.orientation.normalize(); }
}
