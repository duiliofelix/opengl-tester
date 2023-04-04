use gl::types::GLuint;

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new() -> Texture {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
        }

        Texture { id }
    }

    pub fn bind_2d(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn load_2d<P>(&self, path: P)
    where
        P: AsRef<std::path::Path>,
    {
        let img = image::open(path).unwrap();
        let img_data = &(img.as_bytes())[0];

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::mem::transmute(img_data),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn bind_cube(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id);
        }
    }

    pub fn load_cube<P>(
        &self,
        up_path: P,
        down_path: P,
        left_path: P,
        right_path: P,
        front_path: P,
        back_path: P,
    ) where
        P: AsRef<std::path::Path>,
    {
        let pairs = [
            (up_path, gl::TEXTURE_CUBE_MAP_POSITIVE_Y),
            (down_path, gl::TEXTURE_CUBE_MAP_NEGATIVE_Y),
            (left_path, gl::TEXTURE_CUBE_MAP_NEGATIVE_X),
            (right_path, gl::TEXTURE_CUBE_MAP_POSITIVE_X),
            (front_path, gl::TEXTURE_CUBE_MAP_POSITIVE_Z),
            (back_path, gl::TEXTURE_CUBE_MAP_NEGATIVE_Z),
        ];

        for (path, direction) in pairs.into_iter() {
            let img = image::open(path).unwrap();
            let img_data = &(img.as_bytes())[0];

            unsafe {
                gl::TexImage2D(
                    direction,
                    0,
                    gl::RGB as i32,
                    img.width() as i32,
                    img.height() as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    std::mem::transmute(img_data),
                );
            }
        }

        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
        }
    }
}
