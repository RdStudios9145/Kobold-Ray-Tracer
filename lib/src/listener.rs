use crate::Context;
use glfw::WindowEvent;
use std::time::Duration;

pub struct Listener {
    pub(super) on_event:
        Option<Box<dyn Fn(&mut Context, WindowEvent) -> &mut Context>>,
    pub(super) on_update:
        Option<Box<dyn Fn(&mut Context, Duration)    -> &mut Context>>,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            on_event: Option::None,
            on_update: Option::None,
        }
    }
}

impl Listener {
    pub fn new() -> Self {
        Listener::default()
    }
}

impl Listener {
    pub fn attach_on_event(
        &mut self,
        listener: (impl Fn(&mut Context, WindowEvent) -> &mut Context
             + 'static),
    ) -> &mut Self {
        self.on_event = Some(Box::new(listener));
        self
    }

    pub fn attach_on_update(
        &mut self,
        listener: (impl Fn(&mut Context, Duration)    -> &mut Context + 'static),
    ) -> &mut Self {
        self.on_update = Some(Box::new(listener));
        self
    }
}
