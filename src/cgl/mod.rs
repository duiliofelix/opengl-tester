use std::ffi::CString;

pub mod array_object;
pub mod buffer;
pub mod shader;
pub mod texture;

pub use array_object::ArrayObject;
pub use buffer::Buffer;
pub use shader::{Program, Shader};
pub use texture::Texture;

pub struct GLString {
    cstr: CString,
}

impl GLString {
    pub fn from(source: &str) -> GLString {
        GLString {
            cstr: CString::new(source).unwrap(),
        }
    }

    fn as_ptr(&self) -> *const i8 {
        let bytes = self.cstr.as_bytes_with_nul();
        bytes.as_ptr() as *const i8
    }
}
