use gl::types::GLuint;

pub struct ArrayObject {
    id: GLuint,
}

impl ArrayObject {
    pub fn new() -> ArrayObject {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        ArrayObject { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}
