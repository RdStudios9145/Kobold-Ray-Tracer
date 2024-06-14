use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

use crate::{Scene, WindowOptions};

#[derive(Debug)]
pub struct Window {
    opts: WindowOptions,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(opts: WindowOptions, glfw: &mut Glfw) -> Window {
        let (window, events) = glfw
            .create_window(
                opts.width.try_into().unwrap(),
                opts.height.try_into().unwrap(),
                &opts.title,
                glfw::WindowMode::Windowed,
            )
            .expect(&format!("Unable to create window {}", &opts.title));

        Window {
            opts,
            window,
            events,
        }
    }
}

impl Window {
    pub(crate) fn poll_events(&mut self, scenes: &[Scene]) {
        if self.opts.scene >= scenes.len() {
            let _ = glfw::flush_messages(&self.events);
            return;
        }

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                _ => {}
            }
        }
    }

    pub(crate) fn render(&mut self, scenes: &[Scene]) {
        let scene = &scenes[self.opts.scene];
    }
}

impl Window {
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn make_current(&mut self) {
        self.window.make_current()
    }

    pub fn set_all_polling(&mut self, poll: bool) {
        self.window.set_all_polling(poll)
    }
}
