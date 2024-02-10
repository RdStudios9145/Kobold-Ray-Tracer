use crate::camera::Camera;
use crate::scene::Scene;

#[derive(Clone)]
pub struct Context {
    pub current_scene: usize,
    pub camera: Camera,
}

impl Context {
    pub fn current<'a>(&'a self, scenes: &'a mut Vec<Scene>) -> &Scene {
        &scenes[self.current_scene]
    }
}
