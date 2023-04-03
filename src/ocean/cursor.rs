pub struct Cursor {
    pub x: f64,
    pub y: f64,
    pub yaw: f64,
    pub pitch: f64,
    pub sensitivity: f64,
    pub first: bool,
}

impl Cursor {
    pub fn new(window_w: f64, window_h: f64, sensitivity: f64) -> Cursor {
        Cursor {
            x: window_w / 2.,
            y: window_h / 2.,
            yaw: 0.,
            pitch: 0.,
            sensitivity,
            first: true,
        }
    }
}
