#[allow(non_snake_case)]
mod Backend;
mod buffer;
mod camera;
mod object;
mod prelude;
mod quaternion;
mod scene;

use std::time::SystemTime;

use glfw::{fail_on_errors, Glfw, OpenGlProfileHint, WindowHint};
pub use prelude::*;

pub extern crate glfw;
pub extern crate nalgebra_glm as glm;

r#macro::use_backend!(pub, Window);

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

    pub fn register_scene(&mut self, scene: Scene) -> usize {
        let id = self.scenes.len();
        self.scenes.push(scene);
        id
    }

    pub fn create_window(&mut self, opts: WindowOptions) {
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

                let mut marked: Vec<usize> = Vec::new();

                for (i, window) in &mut self.windows.iter_mut().enumerate() {
                    window.poll_events(&mut self.scenes);

                    if window.should_close() {
                        marked.push(i)
                    }
                }

                for i in marked {
                    let window = self.windows.remove(i);
                    window.destroy();
                }

                for window in &mut self.windows {
                    window.update(&mut self.scenes, delta);
                    window.render(&mut self.scenes);
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
        true
    }
}

#[derive(Debug)]
pub struct WindowOptions {
    pub width: usize,
    pub height: usize,
    pub scene: usize,
    pub title: String,
}

pub(crate) mod r#macro {
    #[macro_export]
    macro_rules! _use_backend {
        ($vis: vis, $includes: ident) => {
            #[allow(unused_imports)]
            #[cfg(feature = "vulcan")]
            $vis use $crate::Backend::Vulcan::$includes;

            #[allow(unused_imports)]
            #[cfg(not(feature = "vulcan"))]
            $vis use $crate::Backend::OpenGL::$includes;
        };
    }

    macro_rules! use_backend {
        ($vis: vis, $includes: ident) => {
            _use_backend!($vis, $includes);
        };
        ($includes: ident) => {
            use crate::r#macro::_use_backend;
            _use_backend!(, $includes);
        }
    }

    pub(crate) use _use_backend;
    pub(crate) use use_backend;
}
