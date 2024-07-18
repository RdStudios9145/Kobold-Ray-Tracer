use gl::{
    types::{GLenum, GLuint},
    BindBuffer, BindVertexArray, BufferData, GenBuffers, GenVertexArrays, ARRAY_BUFFER,
    ELEMENT_ARRAY_BUFFER,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = ARRAY_BUFFER as isize,
    ElementArray = ELEMENT_ARRAY_BUFFER as isize,
}

#[derive(Debug)]
pub struct VertexArray(pub GLuint);

pub struct Buffer {
    pub bo: GLuint,
    pub ty: BufferType,
}

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

impl Buffer {
    pub fn new(ty: BufferType) -> Option<Self> {
        let mut bo = 0;

        unsafe { GenBuffers(1, &mut bo) };

        if bo == 0 {
            return None;
        }

        Some(Self { bo, ty })
    }

    pub fn bind(&self) {
        unsafe { BindBuffer(self.ty as GLenum, self.bo) }
    }

    pub fn clear_binding(ty: BufferType) {
        unsafe { BindBuffer(ty as GLenum, 0) }
    }

    pub fn buffer_data(&self, data: &[u8], usage: GLenum) {
        unsafe {
            BufferData(
                self.ty as GLenum,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                usage,
            )
        }
    }
}
