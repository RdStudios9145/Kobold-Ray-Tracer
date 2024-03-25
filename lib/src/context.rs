use crate::camera::Camera;
use crate::scene::Scene;
use glfw::PWindow;

pub struct Context {
    pub current_scene: usize,
    pub scenes: Vec<Scene>,
    pub camera: Camera,
    pub window: PWindow,
}

impl Context {
    pub fn current<'a>(&'a self) -> &Scene {
        &(self.scenes[self.current_scene])
    }
    
    pub fn current_mut(&mut self) -> &mut Scene {
        &mut (self.scenes[self.current_scene])
    }
}
