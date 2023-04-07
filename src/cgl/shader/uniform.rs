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

    pub fn load_glm_vec3(&self, data: &glm::TVec3<f32>) {
        unsafe {
            gl::Uniform3fv(self.id, 0, data.as_ptr());
        }
    }

    pub fn load_vec3(&self, x: f32, y: f32, z: f32) {
        unsafe {
            gl::Uniform3f(self.id, x, y, z);
        }
    }

    pub fn load_i32(&self, data: i32) {
        unsafe {
            gl::Uniform1i(self.id, data);
        }
    }

    pub fn load_f32(&self, data: f32) {
        unsafe {
            gl::Uniform1f(self.id, data);
        }
    }
}
