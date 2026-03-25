use glam::Vec2;
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputState {
    pub dir: Vec2,
    pub is_free_look: bool, 
    pub is_day: bool,
    pub is_flashlight_on: bool,
    pub is_freeform: bool, // NEW: Runtime toggle state
}

impl InputState {
    pub fn new() -> Self { 
        Self { dir: Vec2::ZERO, is_free_look: false, is_day: true, is_flashlight_on: false, is_freeform: false } 
    }

    pub fn process_keyboard(&mut self, key: PhysicalKey, is_pressed: bool) {
        let val = if is_pressed { 1.0 } else { 0.0 };
        match key {
            PhysicalKey::Code(KeyCode::KeyW) => self.dir.y = val,
            PhysicalKey::Code(KeyCode::KeyS) => self.dir.y = -val,
            PhysicalKey::Code(KeyCode::KeyA) => self.dir.x = -val,
            PhysicalKey::Code(KeyCode::KeyD) => self.dir.x = val,
            PhysicalKey::Code(KeyCode::ControlLeft) => self.is_free_look = is_pressed, 
            PhysicalKey::Code(KeyCode::KeyT) => {
                if is_pressed { self.is_day = !self.is_day; } 
            }
            PhysicalKey::Code(KeyCode::KeyF) => {
                if is_pressed { self.is_flashlight_on = !self.is_flashlight_on; }
            }
            PhysicalKey::Code(KeyCode::KeyC) => { // NEW: Toggle camera with 'C'
                if is_pressed { self.is_freeform = !self.is_freeform; }
            }
            _ => {}
        }
    }
}