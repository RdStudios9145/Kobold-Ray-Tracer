use std::sync::{Arc, Mutex};
use std::time::Duration;

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
        let (mut window, events) = glfw
            .create_window(
                opts.width.try_into().unwrap(),
                opts.height.try_into().unwrap(),
                &opts.title,
                glfw::WindowMode::Windowed,
            )
            .unwrap_or_else(|| panic!("Unable to create window {}", &opts.title));

        let win = Arc::new(Mutex::new(&mut window));
        gl::load_with(|s| win.lock().unwrap().get_proc_address(s));

        Window {
            opts,
            window,
            events,
        }
    }
}

impl Window {
    pub(crate) fn poll_events(&mut self, scenes: &mut [Scene]) {
        if self.opts.scene >= scenes.len() {
            let _ = glfw::flush_messages(&self.events);
            return;
        }

        let scene = &mut scenes[self.opts.scene];

        for (_, event) in glfw::flush_messages(&self.events) {
            use glfw::WindowEvent;

            match event {
                WindowEvent::Close => self.window.set_should_close(true),
                WindowEvent::Size(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);

                    if scene.on_event.is_none() {
                        continue;
                    }

                    let on_event = scene.on_event.as_mut().unwrap().clone();
                    (on_event)(&mut self.opts.scene, &mut self.opts.title, scene, event);
                },
                _ => {
                    if scene.on_event.is_none() {
                        continue;
                    }

                    let on_event = scene.on_event.as_mut().unwrap().clone();
                    (on_event)(&mut self.opts.scene, &mut self.opts.title, scene, event);
                }
            }
        }
    }

    pub(crate) fn update(&mut self, scenes: &mut [Scene], delta: Duration) {
        if self.opts.scene >= scenes.len() {
            return;
        }

        let scene = &mut scenes[self.opts.scene];

        if scene.on_update.is_none() {
            return;
        }

        let on_update = scene.on_update.as_mut().unwrap().clone();

        (on_update)(&mut self.opts.scene, &mut self.opts.title, scene, delta);
    }

    pub(crate) fn render(&mut self, scenes: &[Scene]) {
        if self.opts.scene >= scenes.len() {
            return;
        }

        let scene = &scenes[self.opts.scene];

        self.window.make_current();

        if scene.clear_color_dirty {
            let red = scene.clear_color.0;
            let green = scene.clear_color.1;
            let blue = scene.clear_color.2;
            let alpha = scene.clear_color.3;

            unsafe {
                gl::ClearColor(red, green, blue, alpha);
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
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

    pub fn destroy(mut self) {}
}
