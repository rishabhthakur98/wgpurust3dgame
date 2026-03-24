pub mod config;
pub mod colors;

use glam::{Vec2, Vec3};
use crate::render::Vertex;
use crate::world::AABB;

pub struct Player {
    pub pos: Vec3,
}

impl Player {
    pub fn new() -> Self { 
        Self { pos: Vec3::new(config::INIT_POS_X, 0.0, config::INIT_POS_Z) } 
    }

    pub fn update(&mut self, dt: f32, input: Vec2, camera_yaw: f32, limit_x: f32, limit_z: f32, colliders: &[AABB]) {
        let forward = Vec3::new(-camera_yaw.sin(), 0.0, -camera_yaw.cos()).normalize();
        let right = Vec3::new(-forward.z, 0.0, forward.x);

        let move_dir = forward * input.y + right * input.x;
        if move_dir.length_squared() > 0.0 {
            let velocity = move_dir.normalize() * config::SPEED * dt;
            
            // X-Axis Collision check
            let next_x = self.pos.x + velocity.x;
            if !self.is_colliding(next_x, self.pos.z, colliders) {
                self.pos.x = next_x;
            }

            // Z-Axis Collision check
            let next_z = self.pos.z + velocity.z;
            if !self.is_colliding(self.pos.x, next_z, colliders) {
                self.pos.z = next_z;
            }
        }

        // Outer map edge limits
        self.pos.x = self.pos.x.clamp(-limit_x, limit_x);
        self.pos.z = self.pos.z.clamp(-limit_z, limit_z);
    }

    fn is_colliding(&self, x: f32, z: f32, colliders: &[AABB]) -> bool {
        let hs = config::SIZE / 2.0;
        let player_min_x = x - hs;
        let player_max_x = x + hs;
        let player_min_z = z - hs;
        let player_max_z = z + hs;

        for c in colliders {
            if player_max_x > c.min_x && player_min_x < c.max_x &&
               player_max_z > c.min_z && player_min_z < c.max_z {
                return true;
            }
        }
        false
    }
}

pub fn create_vertices() -> Vec<Vertex> {
    let s = config::SIZE / 2.0;
    use colors::*;
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