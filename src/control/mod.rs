use glam::Vec2;
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputState {
    pub dir: Vec2,
}

impl InputState {
    pub fn new() -> Self { Self { dir: Vec2::ZERO } }

    pub fn process_keyboard(&mut self, key: PhysicalKey, is_pressed: bool) {
        let val = if is_pressed { 1.0 } else { 0.0 };
        match key {
            PhysicalKey::Code(KeyCode::KeyW) => self.dir.y = val,
            PhysicalKey::Code(KeyCode::KeyS) => self.dir.y = -val,
            PhysicalKey::Code(KeyCode::KeyA) => self.dir.x = -val,
            PhysicalKey::Code(KeyCode::KeyD) => self.dir.x = val,
            _ => {}
        }
    }
}