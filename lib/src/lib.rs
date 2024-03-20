#![allow(dead_code)]
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
    ops::Deref,
};

use glfw::Context as GContext;
use std::time::Duration;
use std::rc::Weak;

use crate::context::Context;

extern crate nalgebra_glm as glm;

pub use glm::*;
pub use glfw::*;

pub mod buffer;
pub mod camera;
pub mod context;
pub mod flatten;
pub mod object;
pub mod scene;
pub mod shader;
pub mod vertexarray;
pub mod listener;

use shader::ShaderProgram;
use listener::Listener;

pub struct AppData {
    glfw: Glfw,
}

pub struct App {
    pub data: AppData,
}

impl App {
    pub fn new() -> Self {
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
            data: AppData {
                glfw,
            }
        }
    }

    pub fn run<T: Program>(&mut self, program: &mut T, mut context: Context, title: &str, size: (u32, u32)) {
        use gl::*;

        let (mut window, events) = self.data.glfw
            .create_window(size.0, size.1, &title, WindowMode::Windowed)
            .expect("Could not create glfw window and context");

        let win = Arc::new(Mutex::new(&mut window));
        gl::load_with(|s| win.lock().unwrap().get_proc_address(s));

        unsafe {
            gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
        }

        let mut shader = ShaderProgram::from_vert_frag(
            include_str!("default.vert"),
            include_str!("default.frag"),
        ).unwrap();

        program.init(&mut context);

        let mut data = &mut self.data;

        window.make_current();
        window.set_all_polling(true);
        shader.use_program();

        let mut now = SystemTime::now();
        while !window.should_close() {
            let delta = now.elapsed().unwrap();
            now = SystemTime::now();

            // let scene = &mut scenes[self.current_scene as usize];

            data.glfw.poll_events();

            if context.current_scene > context.scenes.len() {
                let _ = glfw::flush_messages(&events);
                continue;
            }

            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Close => window.set_should_close(true),
                    glfw::WindowEvent::ContentScale(w, h) => unsafe {
                        Viewport(0, 0, w as i32, h as i32);
                        program.on_event(event, &mut context);
                    },
                    _ => {
                        program.on_event(event, &mut context);
                    }
                }
            }

            program.on_update(delta, &mut context);

            unsafe {
                Clear(COLOR_BUFFER_BIT);
            }

            // draw stuff
            context.camera.send_to_shader(&shader);
            context.current_mut().render();

            window.swap_buffers();
        }
    }
}

pub trait Program {
    fn on_event(&mut self, ev: WindowEvent, data: &mut Context) {}
    fn on_update(&mut self, ev: Duration, data: &mut Context) {}
    fn init(&mut self, data: &mut Context) {}
}
