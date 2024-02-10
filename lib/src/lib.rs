#![allow(dead_code)]
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use glfw::Context as GContext;
use glfw::{
    fail_on_errors, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowHint, WindowMode,
};

use crate::context::Context;

pub use ::glm::Vector3;
pub use glfw::Key;
pub use glfw::WindowEvent;

pub mod buffer;
pub mod camera;
pub mod context;
pub mod glm;
pub mod object;
pub mod scene;
pub mod shader;
pub mod vertexarray;

use scene::Scene;
use shader::ShaderProgram;

pub struct App {
    window: PWindow,
    glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
    shader: ShaderProgram,
}

impl App {
    pub fn new(title: String, size: (u32, u32)) -> Self {
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

        let (mut window, events) = glfw
            .create_window(size.0, size.1, &title, WindowMode::Windowed)
            .expect("Could not create glfw window and context");

        let win = Arc::new(Mutex::new(&mut window));
        gl::load_with(|s| win.lock().unwrap().get_proc_address(s));

        unsafe {
            gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
        }

        Self {
            window,
            glfw,
            events,
            shader: ShaderProgram::from_vert_frag(
                include_str!("default.vert"),
                include_str!("default.frag"),
            )
            .unwrap(),
        }
    }

    pub fn run(&mut self, mut context: &mut Context, mut scenes: &mut Vec<Scene>) {
        use gl::*;

        self.window.make_current();
        self.window.set_key_polling(true);
        self.shader.use_program();

        let mut now = SystemTime::now();
        while !self.window.should_close() {
            let delta = now.elapsed().unwrap();
            now = SystemTime::now();

            // let scene = &mut self.scenes[self.current_scene as usize];

            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Close => self.window.set_should_close(true),
                    glfw::WindowEvent::ContentScale(w, h) => unsafe {
                        Viewport(0, 0, w as i32, h as i32);
                    },
                    _ => {
                        if context.current_scene > scenes.len() {
                            continue;
                        }

                        if let Some(listener) = &context.current(scenes).on_event {
                            unsafe {
                                (*context, *scenes) = listener(scenes, context, event);
                            }
                        }
                    }
                }
            }

            if context.current_scene > scenes.len() {
                continue;
            }

            unsafe {
                if let Some(listener) = &(*context.current(scenes)).on_update {
                    (*context, *scenes) = listener(scenes, context, delta);
                }
            }

            unsafe {
                Clear(COLOR_BUFFER_BIT);
            }

            // draw stuff
            context.camera.send_to_shader(&self.shader);
            context.current(scenes).render();

            self.window.swap_buffers();
        }
    }
}
