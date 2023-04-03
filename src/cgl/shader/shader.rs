use gl::types::{GLenum, GLuint};
use std::{ffi::CString, fs, ptr};

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

    pub fn compile(&mut self) {
        let shader_code = fs::read_to_string(self.path.clone()).unwrap();
        let shader_code = CString::new(shader_code).unwrap();

        unsafe {
            gl::ShaderSource(self.id, 1, &shader_code.as_ptr(), ptr::null());
            gl::CompileShader(self.id);
        }

        self.assert_ok();
    }

    fn assert_ok(&self) {
        let mut success = 0;

        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut log: Vec<u8> = vec![0; 512];
            let mut len = 0;

            unsafe {
                gl::GetShaderInfoLog(self.id, 512, &mut len, log.as_mut_ptr() as *mut i8);
            }

            println!("{:?}", std::str::from_utf8(&log));
            panic!("DEU ERRO V");
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
