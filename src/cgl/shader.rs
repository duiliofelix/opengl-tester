use gl::types::{GLenum, GLint, GLuint};
use std::{ffi::CString, fs, ptr};

pub struct Shader {
    id: GLuint,
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

pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn new() -> Program {
        let id = unsafe { gl::CreateProgram() };

        Program { id }
    }

    pub fn attach(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
    }

    pub fn link(&self) {
        unsafe {
            gl::LinkProgram(self.id);

            let mut success = 0;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut log: Vec<i8> = vec![0; 512];
                let mut len = 0;
                gl::GetProgramInfoLog(self.id, 512, &mut len, &mut log[0]);
                println!("{:?}", log);
                panic!("DEU ERRO P");
            }
        }
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> GLint {
        unsafe {
            let location = gl::GetUniformLocation(self.id, name.to_string().as_ptr() as *const i8);
            if location == -1 {
                println!("Uniform '{}' not found", name);
            }

            location
        }
    }
}
