use super::Cursor;

type CVec = glm::TVec3<f32>;

pub struct Camera {
    pub pos: CVec,
    pub front: CVec,
    pub up: CVec,
    cursor: Cursor,
}

impl Camera {
    pub fn new(pos: CVec, front: CVec, up: CVec) -> Camera {
        Camera {
            pos,
            front,
            up,
            cursor: Cursor::new(800., 600., 0.01),
        }
    }

    pub fn get_view(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
    }

    pub fn get_right(&self) -> glm::TVec3<f32> {
        glm::cross(&self.front, &self.up)
    }

    pub fn update_cursor(&mut self, new_pos: (f64, f64)) {
        if self.cursor.first {
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
        self.front = glm::normalize(&direction);
    }
}
