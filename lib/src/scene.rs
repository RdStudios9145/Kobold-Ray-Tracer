use crate::prelude::*;
use std::time::Duration;

macro_rules! function {
    ($type: ty) => {
        Option<Box<dyn Fn($type) + 'static>>
    };
}

pub struct Scene {
    objects: Vec<Object>,
    camera: Camera,
    on_update: function!(Duration),
}
