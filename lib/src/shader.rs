use gl::{
    types::{GLenum, GLuint},
    AttachShader, CompileShader, CreateProgram, CreateShader, DeleteProgram, DeleteShader,
    GetProgramInfoLog, GetProgramiv, GetShaderInfoLog, GetShaderiv, LinkProgram, ShaderSource,
    UseProgram, COMPILE_STATUS, COMPUTE_SHADER, FRAGMENT_SHADER, INFO_LOG_LENGTH, LINK_STATUS,
    TRUE, VERTEX_SHADER,
};

pub enum ShaderType {
    Vert = VERTEX_SHADER as isize,
    Frag = FRAGMENT_SHADER as isize,
    Compute = COMPUTE_SHADER as isize,
}

pub struct Shader(pub GLuint);

impl Shader {
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { CreateShader(ty as GLenum) };

        if shader == 0 {
            return None;
        }

        Some(Self(shader))
    }

    pub fn set_source(&self, source: &str) {
        unsafe {
            ShaderSource(
                self.0,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
        }
    }

    pub fn compile(&self) {
        unsafe { CompileShader(self.0) };
    }

    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;

        unsafe { GetShaderiv(self.0, COMPILE_STATUS, &mut compiled) };

        compiled == i32::from(TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;

        unsafe { GetShaderiv(self.0, INFO_LOG_LENGTH, &mut needed_len) };

        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;

        unsafe {
            GetShaderInfoLog(
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
        unsafe { DeleteShader(self.0) };
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

pub struct ShaderProgram(pub GLuint);

impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let prog = unsafe { CreateProgram() };

        if prog == 0 {
            return None;
        }

        Some(Self(prog))
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { AttachShader(self.0, shader.0) };
    }

    pub fn link_program(&self) {
        unsafe { LinkProgram(self.0) };
    }

    pub fn link_success(&self) -> bool {
        let mut success = 0;

        unsafe { GetProgramiv(self.0, LINK_STATUS, &mut success) };

        success == i32::from(TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;

        unsafe { GetProgramiv(self.0, INFO_LOG_LENGTH, &mut needed_len) };

        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;

        unsafe {
            GetProgramInfoLog(
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
        unsafe { UseProgram(self.0) };
    }

    pub fn delete(self) {
        unsafe { DeleteProgram(self.0) };
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
