mod cgl;

extern crate nalgebra_glm as glm;

use cgl::array_object::ArrayObject;
use cgl::buffer::Buffer;
use cgl::shader::{Program, Shader};
use cgl::texture::Texture;
use gl::types::{GLfloat, GLuint};
use glfw::{Action, Context, Key};

const CAMERA_SPEED: f32 = 7.5;

struct Cursor {
    x: f64,
    y: f64,
    yaw: f64,
    pitch: f64,
    sensitivity: f64,
    first: bool,
}

struct Camera {
    pos: glm::TVec3<f32>,
    front: glm::TVec3<f32>,
    up: glm::TVec3<f32>,
    cursor: Cursor,
}

impl Camera {
    fn get_view(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
    }

    fn get_right(&self) -> glm::TVec3<f32> {
        glm::cross(&self.front, &self.up)
    }

    fn update_cursor(&mut self, new_pos: (f64, f64)) {
        if self.cursor.first {
            println!("adjusting first");
            (self.cursor.x, self.cursor.y) = new_pos;
            self.cursor.first = false;
        }

        let offset = (new_pos.0 - self.cursor.x, self.cursor.y - new_pos.1);
        (self.cursor.x, self.cursor.y) = new_pos;

        self.cursor.yaw += offset.0 * self.cursor.sensitivity;
        self.cursor.pitch += offset.1 * self.cursor.sensitivity;

        if self.cursor.yaw >= 89. {
            self.cursor.yaw = 89.
        }
        if self.cursor.pitch <= -89. {
            self.cursor.pitch = -89.
        }

        let direction = glm::vec3(
            (self.cursor.yaw.cos() * self.cursor.pitch.cos()) as f32,
            self.cursor.pitch.sin() as f32,
            (self.cursor.yaw.sin() * self.cursor.pitch.cos()) as f32,
        );
        println!("from: {:?}", self.front);
        self.front = glm::normalize(&direction);
        println!("to: {:?}", self.front);
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello STR", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_pos_polling(true);

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let mut v_shader = Shader::new("res/vertex.glsl", gl::VERTEX_SHADER);
    let mut f_shader = Shader::new("res/fragment.glsl", gl::FRAGMENT_SHADER);
    let mut yellow_f_shader = Shader::new("res/yellow.fragment.glsl", gl::FRAGMENT_SHADER);
    let program = Program::new();
    // let program2 = Program::new();

    v_shader.compile();
    f_shader.compile();
    yellow_f_shader.compile();

    program.attach(&v_shader);
    program.attach(&f_shader);
    program.link();

    /*program2.attach(&v_shader);
    program2.attach(&yellow_f_shader);
    program2.link();*/

    v_shader.delete();
    f_shader.delete();
    yellow_f_shader.delete();

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

    /*unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }*/

    program.activate();
    let model_loc = program.get_uniform_location("model");
    let view_loc = program.get_uniform_location("viewr");
    let projection_loc = program.get_uniform_location("projection");

    let projection = glm::perspective(45.0_f32.to_radians(), 800. / 600., 0.1, 100.);

    unsafe {
        gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

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

    let mut camera = Camera {
        pos: glm::vec3(0., 0., 3.),
        front: glm::vec3(0., 0., -1.),
        up: glm::vec3(0., 1., 0.),
        cursor: Cursor {
            x: 400.,
            y: 300.,
            yaw: 0.,
            pitch: 0.,
            sensitivity: 0.01,
            first: true,
        },
    };

    vao.bind();
    let mut last_time: f32 = 0.;
    while !window.should_close() {
        let time = glfw.get_time() as f32;
        let view = camera.get_view();

        let delta = time - last_time;
        last_time = time;

        unsafe {
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::ClearColor(0.5, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for (index, position) in positions.iter().enumerate() {
            let model = glm::translation(&position);
            let model = glm::rotate(
                &model,
                ((index as f32) * 20.0_f32).to_radians(),
                &glm::vec3(1.0, 0.3, 0.5),
            );
            unsafe {
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
                // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }

        handle_events(&mut glfw, &mut window, &events, &mut camera, delta);

        window.swap_buffers();
    }
}

fn handle_events(
    glfw: &mut glfw::Glfw,
    window: &mut glfw::Window,
    events: &std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    delta: f32,
) {
    let speed = CAMERA_SPEED * delta;

    for (_, event) in glfw::flush_messages(&events) {
        println!("{:?}", event);

        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::Key(Key::W, _, Action::Press | Action::Repeat, _) => {
                camera.pos += speed * camera.front;
            }
            glfw::WindowEvent::Key(Key::S, _, Action::Press | Action::Repeat, _) => {
                camera.pos -= speed * camera.front;
            }
            glfw::WindowEvent::Key(Key::A, _, Action::Press | Action::Repeat, _) => {
                camera.pos -= glm::normalize(&camera.get_right()) * speed;
            }
            glfw::WindowEvent::Key(Key::D, _, Action::Press | Action::Repeat, _) => {
                camera.pos += glm::normalize(&camera.get_right()) * speed;
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                camera.update_cursor((x, y));
            }
            _ => {}
        }
    }
    glfw.poll_events();
}
