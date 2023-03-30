use gl::types::{GLenum, GLsizeiptr, GLuint};
use std::{mem, ptr};

pub struct Buffer<T> {
    id: GLuint,
    buffer_type: GLenum,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn new(buffer_type: GLenum) -> Buffer<T> {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Buffer {
            id,
            buffer_type,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn load_data(&mut self, data: Vec<T>) {
        let type_size = mem::size_of::<T>();

        unsafe {
            gl::BindBuffer(self.buffer_type, self.id);
            gl::BufferData(
                self.buffer_type,
                (data.len() * type_size) as GLsizeiptr,
                mem::transmute(&data[0]),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn set_attrib_format() {
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * mem::size_of::<T>()) as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }
    }
}
