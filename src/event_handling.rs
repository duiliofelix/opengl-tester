use glfw::{Action, Key};

use crate::app_config::AppConfig;
use crate::ocean::Camera;
use glfw::WindowEvent::CursorPos;
use glfw::WindowEvent::Key as KeyEvent;

pub fn handle_events(app: &mut AppConfig, camera: &mut Camera, delta: f32) {
    let speed = camera.speed * delta;

    for (_, event) in glfw::flush_messages(&app.events) {
        match event {
            KeyEvent(Key::Escape, _, Action::Press, _) => {
                app.window.set_should_close(true);
            }
            KeyEvent(Key::W, _, Action::Press | Action::Repeat, _) => {
                camera.pos += speed * camera.front;
            }
            KeyEvent(Key::S, _, Action::Press | Action::Repeat, _) => {
                camera.pos -= speed * camera.front;
            }
            KeyEvent(Key::A, _, Action::Press | Action::Repeat, _) => {
                camera.pos -= glm::normalize(&camera.get_right()) * speed;
            }
            KeyEvent(Key::D, _, Action::Press | Action::Repeat, _) => {
                camera.pos += glm::normalize(&camera.get_right()) * speed;
            }
            CursorPos(x, y) => {
                camera.update_cursor((x, y));
            }
            _ => {}
        }
    }

    app.poll_events();
}
