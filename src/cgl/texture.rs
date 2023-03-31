use gl::types::GLuint;

pub struct Texture {
    id: GLuint,
    img: image::DynamicImage,
}

impl Texture {
    pub fn new<P>(path: P) -> Texture
    where
        P: AsRef<std::path::Path>,
    {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
        }

        let img = image::open(path).unwrap();
        Texture { id, img }
    }

    pub fn bind_2d(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn load(&self) {
        let img_data = &(self.img.as_bytes())[0];

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                self.img.width() as i32,
                self.img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::mem::transmute(img_data),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
}
