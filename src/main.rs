mod app_config;
mod cgl;
mod event_handling;
mod ocean;

extern crate nalgebra_glm as glm;

use app_config::AppConfig;
use cgl::{ArrayObject, Buffer, Program, Shader, Texture};
use gl::types::GLfloat;
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

    let texture = Texture::new();
    texture.bind_2d();
    texture.load_2d("res/wall.jpg");

    let mut v_shader = Shader::new("res/skybox.vertex.glsl", gl::VERTEX_SHADER);
    let mut f_shader = Shader::new("res/skybox.fragment.glsl", gl::FRAGMENT_SHADER);
    let program2 = Program::new();

    v_shader.compile();
    f_shader.compile();

    program2.attach(&v_shader);
    program2.attach(&f_shader);
    program2.link();

    v_shader.delete();
    f_shader.delete();

    let skybox = Texture::new();
    skybox.bind_cube();
    skybox.load_cube(
        "res/skybox/top.jpg",
        "res/skybox/bottom.jpg",
        "res/skybox/left.jpg",
        "res/skybox/right.jpg",
        "res/skybox/front.jpg",
        "res/skybox/back.jpg",
    );

    let skybox_vao = ArrayObject::new();
    skybox_vao.bind();

    let skybox_vbo = Buffer::new(gl::ARRAY_BUFFER);
    let skybox_vertex: Vec<GLfloat> = vec![
        -1.0, 1.0, -1.0, // yaya
        -1.0, -1.0, -1.0, // yaya
        1.0, -1.0, -1.0, // yaya
        1.0, -1.0, -1.0, // yaya
        1.0, 1.0, -1.0, // yaya
        -1.0, 1.0, -1.0, // yaya
        -1.0, -1.0, 1.0, // yaya
        -1.0, -1.0, -1.0, // yaya
        -1.0, 1.0, -1.0, // yaya
        -1.0, 1.0, -1.0, // yaya
        -1.0, 1.0, 1.0, // yaya
        -1.0, -1.0, 1.0, // yaya
        1.0, -1.0, -1.0, // yaya
        1.0, -1.0, 1.0, // yaya
        1.0, 1.0, 1.0, // yaya
        1.0, 1.0, 1.0, // yaya
        1.0, 1.0, -1.0, // yaya
        1.0, -1.0, -1.0, // yaya
        -1.0, -1.0, 1.0, // yaya
        -1.0, 1.0, 1.0, // yaya
        1.0, 1.0, 1.0, // yaya
        1.0, 1.0, 1.0, // yaya
        1.0, -1.0, 1.0, // yaya
        -1.0, -1.0, 1.0, // yaya
        -1.0, 1.0, -1.0, // yaya
        1.0, 1.0, -1.0, // yaya
        1.0, 1.0, 1.0, // yaya
        1.0, 1.0, 1.0, // yaya
        -1.0, 1.0, 1.0, // yaya
        -1.0, 1.0, -1.0, // yaya
        -1.0, -1.0, -1.0, // yaya
        -1.0, -1.0, 1.0, // yaya
        1.0, -1.0, -1.0, // yaya
        1.0, -1.0, -1.0, // yaya
        -1.0, -1.0, 1.0, // yaya
        1.0, -1.0, 1.0, // yaya
    ];
    skybox_vbo.load_data(&skybox_vertex);
    Buffer::set_attrib_format::<GLfloat>(0, 3, 3, 0);

    let mut backpack = ocean::model::Model::new();
    backpack.load_obj();

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

    let model_data = glm::translation(&glm::vec3(0., 0., 2.));
    model.load_mat4(model_data.as_ptr());
    let projection_data = glm::perspective(45.0_f32.to_radians(), 800. / 600., 0.1, 100.);
    projection.load_mat4(projection_data.as_ptr());

    program2.activate();
    let skybox_view = program2
        .get_uniform_location("view")
        .expect("uniform view not found");
    let skybox_projection = program2
        .get_uniform_location("projection")
        .expect("uniform projection not found");
    skybox_projection.load_mat4(projection_data.as_ptr());

    /*unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }*/

    let mut camera = Camera::new(
        glm::vec3(0., 0., 0.),
        glm::vec3(0., 0., -1.),
        glm::vec3(0., 1., 0.),
    );

    let mut last_time: f32 = 0.;
    while !app.window.should_close() {
        let view_data = camera.get_view();
        let box_view = glm::mat3_to_mat4(&glm::mat4_to_mat3(&view_data));

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Disable(gl::DEPTH_TEST);
        }

        program2.activate();
        skybox_view.load_mat4(box_view.as_ptr());
        skybox_vao.bind();
        skybox.bind_cube();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::Enable(gl::DEPTH_TEST);
        }

        program.activate();
        view.load_mat4(view_data.as_ptr());
        backpack.draw();

        let time = app.get_time() as f32;
        let delta = time - last_time;
        last_time = time;

        event_handling::handle_events(&mut app, &mut camera, delta);

        app.window.swap_buffers();
    }
}
