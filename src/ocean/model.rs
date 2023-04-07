use crate::cgl::{ArrayObject, Buffer, Texture};

pub struct Mesh {
    vao: ArrayObject,
    vertex_buffer: Buffer,
    normal_buffer: Buffer,
    tex_pos_buffer: Buffer,
    ebo: Buffer,

    index_count: usize,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vao: ArrayObject::new(),
            vertex_buffer: Buffer::new(gl::ARRAY_BUFFER),
            normal_buffer: Buffer::new(gl::ARRAY_BUFFER),
            tex_pos_buffer: Buffer::new(gl::ARRAY_BUFFER),
            ebo: Buffer::new(gl::ELEMENT_ARRAY_BUFFER),

            index_count: 0,
        }
    }

    pub fn draw(&self) {
        self.vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

pub struct Model {
    meshs: Vec<Mesh>,

    ambient_tex: Texture,
    diffuse_tex: Texture,
    normal_tex: Texture,
    specular_tex: Texture,
}

impl Model {
    pub fn new() -> Model {
        Model {
            meshs: vec![],

            ambient_tex: Texture::new(),
            diffuse_tex: Texture::new(),
            normal_tex: Texture::new(),
            specular_tex: Texture::new(),
        }
    }

    pub fn load_obj(&mut self) {
        let data = tobj::load_obj("res/backpack/backpack.obj", &tobj::GPU_LOAD_OPTIONS);
        let (models, materials) = data.expect("Failed to load OBJ file");
        let materials = materials.expect("NOOO");

        for model in models.iter() {
            let mut mesh = Mesh::new();

            mesh.vao.bind();
            mesh.vertex_buffer.load_data(&model.mesh.positions);
            Buffer::set_attrib_format::<gl::types::GLfloat>(0, 3, 3, 0);
            mesh.tex_pos_buffer.load_data(&model.mesh.normals);
            Buffer::set_attrib_format::<gl::types::GLfloat>(1, 3, 3, 0);
            mesh.tex_pos_buffer.load_data(&model.mesh.texcoords);
            Buffer::set_attrib_format::<gl::types::GLfloat>(2, 2, 2, 0);
            mesh.ebo.load_data(&model.mesh.indices);
            mesh.index_count = model.mesh.indices.len();

            self.meshs.push(mesh);
        }

        self.diffuse_tex.bind_2d();
        self.diffuse_tex.load_2d("res/backpack/diffuse.jpg");
        //self.normal_tex.load_2d("res/backpack/normal.png");
        self.specular_tex.bind_2d();
        self.specular_tex.load_2d("res/backpack/specular.jpg");
    }

    pub fn draw(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
        }
        self.diffuse_tex.bind_2d();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
        }
        self.specular_tex.bind_2d();

        for mesh in self.meshs.iter() {
            mesh.draw();
        }
    }
}
