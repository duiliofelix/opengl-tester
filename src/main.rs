mod app_config;
mod cgl;
mod event_handling;
mod ocean;

extern crate nalgebra_glm as glm;

use app_config::AppConfig;
use cgl::{ArrayObject, Buffer, Program, Shader, Texture};
use gl::types::{GLfloat, GLuint};
use glfw::Context;
use ocean::Camera;

fn main() {
    let mut app = AppConfig::new();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let mut v_shader = Shader::new("res/vertex.glsl", gl::VERTEX_SHADER);
    let mut f_shader = Shader::new("res/fragment.glsl", gl::FRAGMENT_SHADER);
    let program = Program::new();

    v_shader.compile();
    f_shader.compile();

    program.attach(&v_shader);
    program.attach(&f_shader);
    program.link();

    v_shader.delete();
    f_shader.delete();

    let texture = Texture::new("res/wall.jpg");
    texture.bind_2d();
    texture.load();

    let vao = ArrayObject::new();
    vao.bind();

    let mut vbo = Buffer::new(gl::ARRAY_BUFFER);
    let triangle: Vec<GLfloat> = vec![
        -0.5, -0.5, -0.5, 0.0, 0.0, // yaya
        0.5, -0.5, -0.5, 1.0, 0.0, // yaya
        0.5, 0.5, -0.5, 1.0, 1.0, // yaya
        0.5, 0.5, -0.5, 1.0, 1.0, // yaya
        -0.5, 0.5, -0.5, 0.0, 1.0, // yaya
        -0.5, -0.5, -0.5, 0.0, 0.0, // yaya
        -0.5, -0.5, 0.5, 0.0, 0.0, // yaya
        0.5, -0.5, 0.5, 1.0, 0.0, // yaya
        0.5, 0.5, 0.5, 1.0, 1.0, // yaya
        0.5, 0.5, 0.5, 1.0, 1.0, // yaya
        -0.5, 0.5, 0.5, 0.0, 1.0, // yaya
        -0.5, -0.5, 0.5, 0.0, 0.0, // yaya
        -0.5, 0.5, 0.5, 1.0, 0.0, // yaya
        -0.5, 0.5, -0.5, 1.0, 1.0, // yaya
        -0.5, -0.5, -0.5, 0.0, 1.0, // yaya
        -0.5, -0.5, -0.5, 0.0, 1.0, // yaya
        -0.5, -0.5, 0.5, 0.0, 0.0, // yaya
        -0.5, 0.5, 0.5, 1.0, 0.0, // yaya
        0.5, 0.5, 0.5, 1.0, 0.0, // yaya
        0.5, 0.5, -0.5, 1.0, 1.0, // yaya
        0.5, -0.5, -0.5, 0.0, 1.0, // yaya
        0.5, -0.5, -0.5, 0.0, 1.0, // yaya
        0.5, -0.5, 0.5, 0.0, 0.0, // yaya
        0.5, 0.5, 0.5, 1.0, 0.0, // yaya
        -0.5, -0.5, -0.5, 0.0, 1.0, // yaya
        0.5, -0.5, -0.5, 1.0, 1.0, // yaya
        0.5, -0.5, 0.5, 1.0, 0.0, // yaya
        0.5, -0.5, 0.5, 1.0, 0.0, // yaya
        -0.5, -0.5, 0.5, 0.0, 0.0, // yaya
        -0.5, -0.5, -0.5, 0.0, 1.0, // yaya
        -0.5, 0.5, -0.5, 0.0, 1.0, // yaya
        0.5, 0.5, -0.5, 1.0, 1.0, // yaya
        0.5, 0.5, 0.5, 1.0, 0.0, // yaya
        0.5, 0.5, 0.5, 1.0, 0.0, // yaya
        -0.5, 0.5, 0.5, 0.0, 0.0, // yaya
        -0.5, 0.5, -0.5, 0.0, 1.0, // yaya
    ];
    vbo.load_data(triangle);
    Buffer::set_attrib_format::<GLfloat>(0, 3, 5, 0);
    Buffer::set_attrib_format::<GLfloat>(1, 2, 5, 3);

    let mut ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    let indices: Vec<GLuint> = vec![0, 1, 3, 1, 2, 3];
    ebo.load_data(indices);

    program.activate();
    let model = program
        .get_uniform_location("model")
        .expect("uniform model not found");
    let view = program
        .get_uniform_location("view")
        .expect("uniform view not found");
    let projection = program
        .get_uniform_location("projection")
        .expect("uniform projection not found");

    let projection_data = glm::perspective(45.0_f32.to_radians(), 800. / 600., 0.1, 100.);
    projection.load_mat4(projection_data.as_ptr());

    /*unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }*/

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let positions = vec![
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let mut camera = Camera::new(
        glm::vec3(0., 0., 3.),
        glm::vec3(0., 0., -1.),
        glm::vec3(0., 1., 0.),
    );

    vao.bind();
    let mut last_time: f32 = 0.;
    while !app.window.should_close() {
        unsafe {
            gl::ClearColor(0.5, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let view_data = camera.get_view();
        view.load_mat4(view_data.as_ptr());

        let time = app.get_time() as f32;
        let delta = time - last_time;
        last_time = time;

        for (index, position) in positions.iter().enumerate() {
            let model_data = glm::translation(&position);
            let model_data = glm::rotate(
                &model_data,
                ((index as f32) * 20.0_f32).to_radians(),
                &glm::vec3(1.0, 0.3, 0.5),
            );
            model.load_mat4(model_data.as_ptr());
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
                // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }

        event_handling::handle_events(&mut app, &mut camera, delta);

        app.window.swap_buffers();
    }
}
