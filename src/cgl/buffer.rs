use gl::types::{GLenum, GLsizeiptr, GLuint, GLvoid};
use std::mem;

pub struct Buffer {
    id: GLuint,
    buffer_type: GLenum,
}

impl Buffer {
    pub fn new(buffer_type: GLenum) -> Buffer {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Buffer { id, buffer_type }
    }

    pub fn load_data<T>(&self, data: &Vec<T>) {
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

    pub fn set_attrib_format<T>(index: u32, size: i32, total_size: usize, start_pos: usize) {
        let mem_size = (total_size * mem::size_of::<T>()) as i32;
        let start_mem = (start_pos * mem::size_of::<T>()) as *const GLvoid;

        unsafe {
            gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, mem_size, start_mem);
            gl::EnableVertexAttribArray(index);
        }
    }
}
