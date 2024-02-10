use gl::{
    types::{GLenum, GLuint},
    BindBuffer, BufferData, GenBuffers, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = ARRAY_BUFFER as isize,
    ElementArray = ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);

impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;

        unsafe {
            GenBuffers(1, &mut vbo);
        }

        if vbo == 0 {
            return None;
        }

        Some(Self(vbo))
    }

    pub fn bind(&self, ty: BufferType) {
        unsafe {
            BindBuffer(ty as GLenum, self.0);
        }
    }

    pub fn clear_binding(&self, ty: BufferType) {
        unsafe {
            BindBuffer(ty as GLenum, 0);
        }
    }
}

pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        BufferData(
            ty as GLenum,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}
