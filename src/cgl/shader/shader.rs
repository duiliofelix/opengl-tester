use gl::types::{GLenum, GLuint};
use std::{ffi::CString, fs, ptr};

#[derive(Debug)]
pub struct ShaderError {
    compile_msg: String,
}

impl std::error::Error for ShaderError {}
impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shader Error")
    }
}

pub struct Shader {
    pub id: GLuint,
    path: String,
}

impl Shader {
    pub fn new(path: &str, shader_type: GLenum) -> Shader {
        let id = unsafe { gl::CreateShader(shader_type) };

        Shader {
            id,
            path: String::from(path),
        }
    }

    pub fn compile(&mut self) -> Result<(), ShaderError> {
        let shader_code = fs::read_to_string(self.path.clone()).unwrap();
        let shader_code = CString::new(shader_code).unwrap();

        unsafe {
            gl::ShaderSource(self.id, 1, &shader_code.as_ptr(), ptr::null());
            gl::CompileShader(self.id);
        }

        self.assert_ok()
    }

    fn assert_ok(&self) -> Result<(), ShaderError> {
        let mut success = 0;

        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        }

        if success != 0 {
            return Ok(());
        }

        let mut log: Vec<u8> = vec![0; 512];
        let mut len = 0;

        unsafe {
            gl::GetShaderInfoLog(self.id, 512, &mut len, log.as_mut_ptr() as *mut i8);
        }

        let clear_log: Vec<u8> = log.iter().copied().filter(|x| *x != 0).collect();
        let c_str = CString::new(clear_log).unwrap();
        let error = ShaderError {
            compile_msg: c_str.into_string().unwrap(),
        };

        Err(error)
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
