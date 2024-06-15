use glfw::WindowEvent;
use glm::Vec3;

use crate::{Camera, Object, Quaternion};
use std::{sync::Arc, time::Duration};

macro_rules! function {
    ($type: ty) => {
        Option<Arc<dyn Fn(&mut usize, &mut str, &mut Scene, $type) + 'static>>
    };
}

pub struct Scene {
    objects: Vec<Object>,
    camera: Camera,
    pub(crate) on_update: function!(Duration),
    pub(crate) on_event: function!(WindowEvent),
}

impl Scene {
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
}
