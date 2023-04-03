use crate::cgl::GLString;

use super::{Shader, Uniform};
use gl::types::GLuint;

#[derive(Debug)]
pub enum ProgramError {
    UniformNotFound,
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

    pub fn get_uniform_location(&self, name: &str) -> Result<Uniform, ProgramError> {
        let gl_name = GLString::from(name);
        let location = unsafe { gl::GetUniformLocation(self.id, gl_name.as_ptr()) };

        if location == -1 {
            Err(ProgramError::UniformNotFound)
        } else {
            Ok(Uniform { id: location })
        }
    }
}
