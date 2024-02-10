use gl::{types::GLuint, BindVertexArray, GenVertexArrays};

pub struct VertexArray(pub GLuint);

impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao = 0;

        unsafe { GenVertexArrays(1, &mut vao) };

        if vao == 0 {
            return None;
        }

        Some(Self(vao))
    }

    pub fn bind(&self) {
        unsafe { BindVertexArray(self.0) }
    }

    pub fn clear_binding() {
        unsafe { BindVertexArray(0) }
    }
}
