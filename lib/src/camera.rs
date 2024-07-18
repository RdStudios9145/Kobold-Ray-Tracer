use crate::Quaternion;
use glm::{identity, perspective, translate, vec3, Mat4, Vec3};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Camera {
    pub(super) view: Mat4,
    pub(super) projection: Mat4,
    pub orientation: Quaternion,
    pub position: Vec3,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            view: identity(),
            projection: perspective(aspect, 45.0, 0.1, 100.0),
            orientation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
            position: vec3(0., 0., 0.),
        }
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        self.position += translation;
        let translation = translation * -1.0;
        self.view = translate(&self.view, &translation);
        self
    }

    pub fn rotate(&mut self, rotation: Quaternion) -> &mut Self {
        self.orientation *= rotation;
        self.orientation.normalize();
        self
    }
}
