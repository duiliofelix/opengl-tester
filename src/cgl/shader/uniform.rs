use gl::types::GLint;

#[derive(Debug)]
pub struct Uniform {
    pub id: GLint,
}

impl Uniform {
    pub fn load_mat4(&self, data: *const f32) {
        unsafe {
            gl::UniformMatrix4fv(self.id, 1, gl::FALSE, data);
        }
    }
}
