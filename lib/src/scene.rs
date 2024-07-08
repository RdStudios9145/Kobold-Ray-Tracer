use glfw::WindowEvent;
use glm::Vec3;

use crate::{Camera, Object, Quaternion};
use std::{sync::Arc, time::Duration};

macro_rules! function {
    ($type: ty) => {
        //                Current Window Scene ID, Window Title, Current Scene
        Option<Arc<dyn Fn(&mut usize,              &mut str,     &mut Scene,    $type) + 'static>>
    };
}

// Scenes are not marked as dirty because they need window specific information. If there are
// multiple windows open on one scene, the scene will be updated twice
pub struct Scene {
    objects: Vec<Object>,
    camera: Camera,
    pub(crate) on_update: function!(Duration),
    pub(crate) on_event: function!(WindowEvent),
    pub(crate) clear_color: (f32, f32, f32, f32),
    pub(crate) clear_color_dirty: bool,
}

impl Scene {
    pub fn new(aspect: f32) -> Self {
        Self {
            objects: Vec::new(),
            camera: Camera::new(aspect),
            on_update: None,
            on_event: None,
            clear_color: (0., 0., 0., 0.),
            clear_color_dirty: false,
        }
    }

    pub fn add_object(
        &mut self,
        id: usize,
        position: Vec3,
        scale: Vec3,
        rotation: Quaternion,
    ) -> usize {
        let index = self.objects.len();
        self.objects.push(Object {
            object_type: id,
            position,
            scale,
            orientation: rotation,
        });
        index
    }

    pub fn set_clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.clear_color = (red, green, blue, alpha);
        self.clear_color_dirty = true;
    }
}
