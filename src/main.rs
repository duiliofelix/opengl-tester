mod cgl;

use cgl::array_object::ArrayObject;
use cgl::buffer::Buffer;
use cgl::shader::{Program, Shader};
use gl::types::{GLfloat, GLubyte, GLuint, GLvoid};
use glfw::{Action, Context, Key};
use std::{mem, ptr};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello STR", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);

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

    let tex_image = image::open("res/wall.jpg").unwrap();

    let mut texture: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            tex_image.width() as i32,
            tex_image.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            mem::transmute(&(tex_image.into_bytes())[0]),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let vao = ArrayObject::new();
    vao.bind();

    let mut vbo = Buffer::new(gl::ARRAY_BUFFER);
    let triangle: Vec<GLfloat> = vec![
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
    ];
    vbo.load_data(triangle);
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * mem::size_of::<GLfloat>()) as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * mem::size_of::<GLfloat>()) as i32,
            (3 * mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * mem::size_of::<GLfloat>()) as i32,
            (6 * mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);
    }

    let mut ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    let indices: Vec<GLuint> = vec![0, 1, 3, 1, 2, 3];
    ebo.load_data(indices);

    /*unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }*/

    program.activate();
    unsafe {
        let location =
            gl::GetUniformLocation(program.id, "xOffset".to_string().as_ptr() as *const i8);
        if location == -1 {
            println!("NOOOOO");
        }
        gl::Uniform1f(location, 0.5);
    }

    vao.bind();
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.5, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        handle_events(&mut glfw, &mut window, &events);

        window.swap_buffers();
    }
}

fn handle_events(
    glfw: &mut glfw::Glfw,
    window: &mut glfw::Window,
    events: &std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
) {
    for (_, event) in glfw::flush_messages(&events) {
        println!("{:?}", event);

        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            _ => {}
        }
    }
    glfw.poll_events();
}
