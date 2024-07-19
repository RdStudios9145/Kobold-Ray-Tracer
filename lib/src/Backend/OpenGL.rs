#![allow(dead_code)]
use std::ffi::CString;
use std::mem::size_of;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gl::types::{GLenum, GLuint};
use gl::{
    COMPILE_STATUS, COMPUTE_SHADER, FALSE, FLOAT, FRAGMENT_SHADER, INFO_LOG_LENGTH, LINK_STATUS,
    STATIC_DRAW, TRIANGLES, TRUE, UNSIGNED_INT, VERTEX_SHADER,
};
use glfw::{Context, CursorMode, Glfw, GlfwReceiver, PWindow, WindowEvent};
use glm::{Mat4, Vec3, Vec4};

use crate::buffer::{Buffer, BufferType, VertexArray};
use crate::{Camera, ObjectManager, Scene, TriangleIndecies, Vertex, WindowOptions};

#[derive(Debug)]
pub struct Window {
    opts: WindowOptions,
    window: PWindow,
    events: Arc<GlfwReceiver<(f64, WindowEvent)>>,
    shader: ShaderProgram,
    pub object_manager: ObjectManager,
}

#[derive(Debug)]
pub(crate) struct ObjectInformation {
    vao: VertexArray,
    inst_vbo: Buffer,
}

pub enum ShaderType {
    Vert = VERTEX_SHADER as isize,
    Frag = FRAGMENT_SHADER as isize,
    Compute = COMPUTE_SHADER as isize,
}

struct Shader(pub GLuint);
#[derive(Debug)]
struct ShaderProgram(pub GLuint);

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

        let shader = ShaderProgram::from_vert_frag(
            include_str!("vertex.glsl"),
            include_str!("fragment.glsl"),
        )
        .unwrap();

        unsafe { gl::Enable(gl::DEPTH_TEST) };
        glfw.set_swap_interval(glfw::SwapInterval::None);

        Window {
            opts,
            window,
            events: Arc::new(events),
            shader,
            object_manager: ObjectManager::new(),
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

        for (_, event) in glfw::flush_messages(&self.events.clone()) {
            use glfw::WindowEvent;

            match event {
                WindowEvent::Close => self.window.set_should_close(true),
                WindowEvent::Size(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);

                    if scene.on_event.is_none() {
                        continue;
                    }

                    let on_event = scene.on_event.as_mut().unwrap().clone();
                    (on_event.lock().unwrap())(self, scene, event);
                },
                _ => {
                    if scene.on_event.is_none() {
                        continue;
                    }

                    let on_event = scene.on_event.as_mut().unwrap().clone();
                    (on_event.lock().unwrap())(self, scene, event);
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

        (on_update.lock().unwrap())(self, scene, delta);
    }

    pub(crate) fn render(&mut self, scenes: &mut [Scene]) {
        if self.opts.scene >= scenes.len() {
            return;
        }

        let scene = &mut scenes[self.opts.scene];

        self.window.make_current();
        self.shader.use_program();

        self.send_camera_info(&scene.camera);

        if scene.clear_color_dirty {
            let c = scene.clear_color;

            unsafe {
                gl::ClearColor(c.0, c.1, c.2, c.3);
            }

            scene.clear_color_dirty = false;
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for obj in &scene.objects {
            let ty = self.object_manager.from_id(obj.object_type);
            ty.info.vao.bind();

            //let mut position_mat = Mat4::identity();
            //position_mat = glm::translate(&position_mat, &obj.position);

            //let rotation_mat = obj.orientation.as_matrix();

            //let mut scale_mat = Mat4::identity();
            //scale_mat = glm::scale(&scale_mat, &obj.scale);

            //self.send_matrix("obj_mat", &(position_mat * rotation_mat * scale_mat));

            self.send_vec3("obj_position", &obj.position);
            self.send_vec3("obj_scale", &obj.scale);
            self.send_matrix("obj_rotation", &obj.orientation.as_matrix());

            self.send_vec4("color", &obj.color);

            unsafe {
                gl::DrawElements(
                    TRIANGLES,
                    ty.tris.len() as i32 * 3,
                    UNSIGNED_INT,
                    //0 as *const _,
                    std::ptr::null(),
                )
            };
        }

        self.window.swap_buffers();
    }

    fn send_camera_info(&self, camera: &Camera) {
        self.send_matrix("cam_view", &camera.view);
        self.send_matrix("cam_projection", &camera.projection);
        self.send_matrix("cam_orientation", &camera.orientation.as_matrix())
    }

    fn uniform_location(&self, name: &str) -> i32 {
        let cname = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.shader.0, cname.as_ptr() as *const i8) }
    }

    fn send_matrix(&self, name: &str, mat: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_location(name),
                1,
                FALSE,
                mat.as_slice().as_ptr(),
            )
        }
    }

    fn send_vec4(&self, name: &str, vec: &Vec4) {
        unsafe { gl::Uniform4fv(self.uniform_location(name), 1, vec.as_ptr()) }
    }

    fn send_vec3(&self, name: &str, vec: &Vec3) {
        unsafe { gl::Uniform3fv(self.uniform_location(name), 1, vec.as_ptr()) }
    }
}

impl ObjectInformation {
    pub fn new(obj: (&[Vertex], &[TriangleIndecies])) -> Option<Self> {
        let vao = VertexArray::new()?;
        vao.bind();

        let vbo = Buffer::new(BufferType::Array)?;
        vbo.bind();
        vbo.buffer_data(bytemuck::cast_slice(obj.0), STATIC_DRAW);

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                FLOAT,
                FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
                //0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
        }

        let ebo = Buffer::new(BufferType::ElementArray)?;
        ebo.bind();
        ebo.buffer_data(bytemuck::cast_slice(obj.1), STATIC_DRAW);

        let inst_vbo = Buffer::new(BufferType::Array)?;
        inst_vbo.bind();

        unsafe {
            //gl::VertexAttribPointer(
            //    1,
            //    4,
            //    FLOAT,
            //    FALSE,
            //    4 * size_of::<Vec4>().try_into().unwrap(),
            //    0 as *const _,
            //);
            //gl::EnableVertexAttribArray(1);
            //gl::VertexAttribPointer(
            //    2,
            //    4,
            //    FLOAT,
            //    FALSE,
            //    4 * size_of::<Vec4>().try_into().unwrap(),
            //    size_of::<Vec4>() as *const _,
            //);
            //gl::EnableVertexAttribArray(2);
            //gl::VertexAttribPointer(
            //    3,
            //    4,
            //    FLOAT,
            //    FALSE,
            //    4 * size_of::<Vec4>().try_into().unwrap(),
            //    (2 * size_of::<Vec4>()) as *const _,
            //);
            //gl::EnableVertexAttribArray(3);
            //gl::VertexAttribPointer(
            //    4,
            //    4,
            //    FLOAT,
            //    FALSE,
            //    4 * size_of::<Vec4>().try_into().unwrap(),
            //    (3 * size_of::<Vec4>()) as *const _,
            //);
            //gl::EnableVertexAttribArray(4);
        }

        Some(Self { vao, inst_vbo })
    }
}

impl Window {
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }

    pub fn make_current(&mut self) {
        self.window.make_current()
    }

    pub fn set_all_polling(&mut self, poll: bool) {
        self.window.set_all_polling(poll)
    }

    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.window.set_cursor_mode(mode)
    }

    pub fn destroy(self) {}
}

impl Shader {
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { gl::CreateShader(ty as GLenum) };

        if shader == 0 {
            return None;
        }

        Some(Self(shader))
    }

    pub fn set_source(&self, source: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
        }
    }

    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.0) };
    }

    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;

        unsafe { gl::GetShaderiv(self.0, COMPILE_STATUS, &mut compiled) };

        compiled == i32::from(TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;

        unsafe { gl::GetShaderiv(self.0, INFO_LOG_LENGTH, &mut needed_len) };

        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;

        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );

            v.set_len(len_written.try_into().unwrap());
        }

        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn delete(self) {
        unsafe { gl::DeleteShader(self.0) };
    }

    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty).ok_or_else(|| "Couldn't allocate new shader".to_string())?;

        id.set_source(source);
        id.compile();

        if id.compile_success() {
            return Ok(id);
        }

        let out = id.info_log();
        id.delete();
        Err(out)
    }
}

impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let prog = unsafe { gl::CreateProgram() };

        if prog == 0 {
            return None;
        }

        Some(Self(prog))
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.0, shader.0) };
    }

    pub fn link_program(&self) {
        unsafe { gl::LinkProgram(self.0) };
    }

    pub fn link_success(&self) -> bool {
        let mut success = 0;

        unsafe { gl::GetProgramiv(self.0, LINK_STATUS, &mut success) };

        success == i32::from(TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;

        unsafe { gl::GetProgramiv(self.0, INFO_LOG_LENGTH, &mut needed_len) };

        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;

        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );

            v.set_len(len_written.try_into().unwrap());
        }

        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.0) };
    }

    pub fn delete(self) {
        unsafe { gl::DeleteProgram(self.0) };
    }

    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vert, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;
        let f = Shader::from_source(ShaderType::Frag, frag)
            .map_err(|e| format!("Fragment Compile Error: {}", e))?;

        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.delete();
        f.delete();

        if p.link_success() {
            return Ok(p);
        }

        let out = format!("Program Link Error: {}", p.info_log());
        p.delete();
        Err(out)
    }
}
