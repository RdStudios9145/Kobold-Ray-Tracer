use std::mem::size_of;

use gl::ClearColor;
use gl::{
    types::GLfloat, DrawElements, EnableVertexAttribArray, VertexAttribPointer, FALSE, FLOAT,
    STATIC_DRAW, TRIANGLES, UNSIGNED_INT,
};

use crate::buffer::{buffer_data, Buffer, BufferType};
use crate::object::Object;
use crate::object::Vertex;
use crate::vertexarray::VertexArray;

pub type Color = (GLfloat, GLfloat, GLfloat, GLfloat);

pub struct Scene {
    pub objects: Vec<Object>,
    pub(super) vaos: Vec<VertexArray>,
    pub(super) vbos: Vec<Buffer>,
    pub(super) ebos: Vec<Buffer>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            vaos: Vec::new(),
            vbos: Vec::new(),
            ebos: Vec::new(),
        }
    }

    pub fn render(&mut self) {
        for object in self.objects.iter_mut() {
            if !object.ids_generated {
                let vao = VertexArray::new().unwrap();
                vao.bind();

                let vbo = Buffer::new().unwrap();
                vbo.bind(BufferType::Array);
                buffer_data(
                    BufferType::Array,
                    bytemuck::cast_slice(&object.verts),
                    STATIC_DRAW,
                );

                unsafe {
                    VertexAttribPointer(
                        0,
                        3,
                        FLOAT,
                        FALSE,
                        size_of::<Vertex>().try_into().unwrap(),
                        0 as *const _,
                    );
                    EnableVertexAttribArray(0);
                }

                let ebo = Buffer::new().unwrap();
                ebo.bind(BufferType::ElementArray);
                buffer_data(
                    BufferType::ElementArray,
                    bytemuck::cast_slice(&object.indicies),
                    STATIC_DRAW,
                );

                self.vaos.push(vao);
                self.vbos.push(vbo);
                self.ebos.push(ebo);

                object.vao_id = self.vaos.len() as u32 - 1;
                object.vbo_id = self.vbos.len() as u32 - 1;
                object.ebo_id = self.ebos.len() as u32 - 1;

                object.ids_generated = true;
            }

            self.vaos[object.vao_id as usize].bind();

            unsafe {
                DrawElements(
                    TRIANGLES,
                    object.indicies.len() as i32 * 3,
                    // 6,
                    UNSIGNED_INT,
                    0 as *const _,
                );
            }
        }
    }
}

impl Scene {
    pub fn add_object(&mut self, object: Object) -> &mut Self {
        self.objects.push(object);

        self
    }

    pub fn set_refresh_color(&mut self, color: Color) -> &mut Self {
        unsafe {
            ClearColor(color.0, color.1, color.2, color.3);
        }

        self
    }
}
