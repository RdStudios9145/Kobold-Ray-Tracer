use crate::camera::Camera;
use crate::scene::Scene;

pub struct Context {
    pub current_scene: usize,
    pub scenes: Vec<Scene>,
    pub camera: Camera,
}

impl Context {
    pub fn current<'a>(&'a self) -> &Scene {
        &(self.scenes[self.current_scene])
    }
    
    pub fn current_mut(&mut self) -> &mut Scene {
        &mut (self.scenes[self.current_scene])
    }
}
