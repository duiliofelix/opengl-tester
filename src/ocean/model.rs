use crate::cgl::{ArrayObject, Buffer, Texture};

pub struct Mesh {
    vao: ArrayObject,
    vbo: Buffer,
    color: Buffer,
    ebo: Buffer,

    index_count: usize,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vao: ArrayObject::new(),
            vbo: Buffer::new(gl::ARRAY_BUFFER),
            color: Buffer::new(gl::ARRAY_BUFFER),
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
            mesh.vbo.load_data(&model.mesh.positions);
            Buffer::set_attrib_format::<gl::types::GLfloat>(0, 3, 3, 0);
            mesh.color.load_data(&model.mesh.texcoords);
            Buffer::set_attrib_format::<gl::types::GLfloat>(1, 2, 2, 0);
            mesh.ebo.load_data(&model.mesh.indices);
            mesh.index_count = model.mesh.indices.len();

            self.meshs.push(mesh);
        }

        println!("{:?}", materials[0].diffuse_texture);
        self.diffuse_tex.bind_2d();
        self.diffuse_tex.load_2d("res/backpack/diffuse.jpg");
        //self.normal_tex.load_2d("res/backpack/normal.png");
        //self.specular_tex.load_2d("res/backpack/specular.jpg");
    }

    pub fn draw(&self) {
        self.diffuse_tex.bind_2d();

        for mesh in self.meshs.iter() {
            mesh.draw();
        }
    }
}
