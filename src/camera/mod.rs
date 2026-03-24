pub mod config;

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self { yaw: 0.0, pitch: 0.5, distance: config::DEFAULT_DISTANCE }
    }
    pub fn process_mouse(&mut self, dx: f32, dy: f32) {
        self.yaw -= dx * config::SENSITIVITY;
        self.pitch += dy * config::SENSITIVITY;
        self.pitch = self.pitch.clamp(config::PITCH_MIN, config::PITCH_MAX);
    }
}