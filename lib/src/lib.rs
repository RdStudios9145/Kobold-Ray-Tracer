#[allow(non_snake_case)]
mod Backend;
mod camera;
mod object;
mod prelude;
mod quaternion;
mod scene;

use std::time::SystemTime;

use glfw::{fail_on_errors, Glfw, OpenGlProfileHint, WindowHint};
use prelude::*;

r#macro::use_backend!(Window);

pub struct App {
    scenes: Vec<Scene>,
    windows: Vec<Window>,
    glfw: Glfw,
}

impl App {
    pub fn new(scenes: Vec<Scene>) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        glfw.window_hint(WindowHint::ContextVersionMajor(4));
        glfw.window_hint(WindowHint::ContextVersionMinor(6));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        if cfg!(target_os = "macos") {
            glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        }

        if cfg!(debug_asserts) {
            glfw.window_hint(WindowHint::OpenGlDebugContext(true));
        }

        Self {
            scenes,
            windows: Vec::new(),
            glfw,
        }
    }

    pub fn creat_window(&mut self, opts: WindowOptions) {
        let mut window = Window::new(opts, &mut self.glfw);

        window.make_current();
        window.set_all_polling(true);

        self.windows.push(window);
    }

    pub fn run(mut self) {
        let mut now = SystemTime::now();
        while !self.should_close() {
            let delta = now.elapsed().unwrap();
            now = SystemTime::now();

            if !self.windows.is_empty() {
                self.glfw.poll_events();

                for window in &mut self.windows {
                    window.poll_events(&self.scenes);
                }

                for window in &mut self.windows {
                    window.render(&self.scenes);
                }
            }
        }
    }

    fn should_close(&self) -> bool {
        // if there are no windows open, app can quit
        if self.windows.is_empty() {
            return true;
        }

        for window in &self.windows {
            // If there is still a window open, dont close
            if !window.should_close() {
                return false;
            }
        }

        // something is going wrong if this is the return point, safe to quit
        return true;
    }
}

#[derive(Debug)]
pub struct WindowOptions {
    pub width: usize,
    pub height: usize,
    pub scene: usize,
    pub title: String,
}

mod r#macro {
    macro_rules! use_backend {
        ($includes: ident) => {
            #[allow(unused_imports)]
            #[cfg(feature = "vulcan")]
            use Backend::Vulcan::$includes;

            #[allow(unused_imports)]
            #[cfg(not(feature = "vulcan"))]
            use Backend::OpenGL::$includes;
        };
    }

    pub(crate) use use_backend;
}
