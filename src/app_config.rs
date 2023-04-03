use glfw::{Context, Glfw, Window};

type EventReceiver = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

pub struct AppConfig {
    glfw: Glfw,
    pub window: Window,
    pub events: EventReceiver,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(800, 600, "Hello STR", glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_cursor_pos_polling(true);

        gl::load_with(|s| glfw.get_proc_address_raw(s));

        AppConfig {
            glfw,
            window,
            events,
        }
    }

    pub fn get_time(&self) -> f32 {
        self.glfw.get_time() as f32
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
}
