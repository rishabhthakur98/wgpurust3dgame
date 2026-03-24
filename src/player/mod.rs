pub mod config;
use glam::{Vec2, Vec3};
use crate::core::colors::*;
use crate::render::Vertex;

pub struct Player {
    pub pos: Vec3,
}

impl Player {
    pub fn new() -> Self { Self { pos: Vec3::ZERO } }

    pub fn update(&mut self, dt: f32, input: Vec2, camera_yaw: f32, limit: f32) {
        let forward = Vec3::new(-camera_yaw.sin(), 0.0, -camera_yaw.cos()).normalize();
        let right = Vec3::new(-forward.z, 0.0, forward.x);

        let move_dir = forward * input.y + right * input.x;
        if move_dir.length_squared() > 0.0 {
            self.pos += move_dir.normalize() * config::SPEED * dt;
        }

        self.pos.x = self.pos.x.clamp(-limit, limit);
        self.pos.z = self.pos.z.clamp(-limit, limit);
    }
}

pub fn create_vertices() -> Vec<Vertex> {
    let s = config::SIZE / 2.0;
    vec![
        Vertex { position: [-s, s, -s], color: RED }, Vertex { position: [-s, s, s], color: RED }, Vertex { position: [s, s, -s], color: RED },
        Vertex { position: [-s, s, s], color: RED }, Vertex { position: [s, s, s], color: RED }, Vertex { position: [s, s, -s], color: RED },
        Vertex { position: [-s, -s, s], color: LIGHT_GREEN }, Vertex { position: [s, -s, s], color: LIGHT_GREEN }, Vertex { position: [-s, s, s], color: LIGHT_GREEN },
        Vertex { position: [s, -s, s], color: LIGHT_GREEN }, Vertex { position: [s, s, s], color: LIGHT_GREEN }, Vertex { position: [-s, s, s], color: LIGHT_GREEN },
        Vertex { position: [s, -s, s], color: LIGHT_ORANGE }, Vertex { position: [s, -s, -s], color: LIGHT_ORANGE }, Vertex { position: [s, s, s], color: LIGHT_ORANGE },
        Vertex { position: [s, -s, -s], color: LIGHT_ORANGE }, Vertex { position: [s, s, -s], color: LIGHT_ORANGE }, Vertex { position: [s, s, s], color: LIGHT_ORANGE },
        Vertex { position: [s, -s, -s], color: LIGHT_BLUE }, Vertex { position: [-s, -s, -s], color: LIGHT_BLUE }, Vertex { position: [s, s, -s], color: LIGHT_BLUE },
        Vertex { position: [-s, -s, -s], color: LIGHT_BLUE }, Vertex { position: [-s, s, -s], color: LIGHT_BLUE }, Vertex { position: [s, s, -s], color: LIGHT_BLUE },
        Vertex { position: [-s, -s, -s], color: LIGHT_YELLOW }, Vertex { position: [-s, -s, s], color: LIGHT_YELLOW }, Vertex { position: [-s, s, -s], color: LIGHT_YELLOW },
        Vertex { position: [-s, -s, s], color: LIGHT_YELLOW }, Vertex { position: [-s, s, s], color: LIGHT_YELLOW }, Vertex { position: [-s, s, -s], color: LIGHT_YELLOW },
        Vertex { position: [-s, -s, s], color: DARK_GREY }, Vertex { position: [-s, -s, -s], color: DARK_GREY }, Vertex { position: [s, -s, s], color: DARK_GREY },
        Vertex { position: [-s, -s, -s], color: DARK_GREY }, Vertex { position: [s, -s, -s], color: DARK_GREY }, Vertex { position: [s, -s, s], color: DARK_GREY },
    ]
}