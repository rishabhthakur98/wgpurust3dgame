pub mod config;
pub mod colors;

use glam::{Vec2, Vec3};
use crate::render::Vertex;
use crate::world::AABB;

pub struct Player {
    pub pos: Vec3,
    pub yaw: f32, // Give the player its own rotation
}

impl Player {
    pub fn new() -> Self { 
        Self { pos: Vec3::new(config::INIT_POS_X, 0.0, config::INIT_POS_Z), yaw: 0.0 } 
    }

    pub fn update(&mut self, dt: f32, input: Vec2, camera_yaw: f32, is_free_look: bool, limit_x: f32, limit_z: f32, colliders: &[AABB]) {
        // If holding Right Ctrl, do not move and do not rotate the player's body
        if is_free_look {
            return; 
        }

        // Snap player rotation to camera rotation when not in free-look
        self.yaw = camera_yaw;

        let forward = Vec3::new(-camera_yaw.sin(), 0.0, -camera_yaw.cos()).normalize();
        let right = Vec3::new(-forward.z, 0.0, forward.x);

        let move_dir = forward * input.y + right * input.x;
        if move_dir.length_squared() > 0.0 {
            let velocity = move_dir.normalize() * config::SPEED * dt;
            
            let next_x = self.pos.x + velocity.x;
            if !self.is_colliding(next_x, self.pos.z, colliders) {
                self.pos.x = next_x;
            }

            let next_z = self.pos.z + velocity.z;
            if !self.is_colliding(self.pos.x, next_z, colliders) {
                self.pos.z = next_z;
            }
        }

        self.pos.x = self.pos.x.clamp(-limit_x, limit_x);
        self.pos.z = self.pos.z.clamp(-limit_z, limit_z);
    }

    fn is_colliding(&self, x: f32, z: f32, colliders: &[AABB]) -> bool {
        let hs = config::SIZE / 2.0;
        let player_min_x = x - hs;
        let player_max_x = x + hs;
        let player_min_z = z - hs;
        let player_max_z = z + hs;
        
        let player_min_y = 0.0;
        let player_max_y = config::SIZE;

        for c in colliders {
            if player_max_x > c.min_x && player_min_x < c.max_x &&
               player_max_y > c.min_y && player_min_y < c.max_y &&
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
    let up = [0.0, 1.0, 0.0]; let down = [0.0, -1.0, 0.0];
    let front = [0.0, 0.0, 1.0]; let back = [0.0, 0.0, -1.0];
    let right = [1.0, 0.0, 0.0]; let left = [-1.0, 0.0, 0.0];

    vec![
        // Top Face
        Vertex { position: [-s, s, -s], color: RED, normal: up }, Vertex { position: [-s, s, s], color: RED, normal: up }, Vertex { position: [s, s, -s], color: RED, normal: up },
        Vertex { position: [-s, s, s], color: RED, normal: up }, Vertex { position: [s, s, s], color: RED, normal: up }, Vertex { position: [s, s, -s], color: RED, normal: up },
        // Front Face
        Vertex { position: [-s, -s, s], color: LIGHT_GREEN, normal: front }, Vertex { position: [s, -s, s], color: LIGHT_GREEN, normal: front }, Vertex { position: [-s, s, s], color: LIGHT_GREEN, normal: front },
        Vertex { position: [s, -s, s], color: LIGHT_GREEN, normal: front }, Vertex { position: [s, s, s], color: LIGHT_GREEN, normal: front }, Vertex { position: [-s, s, s], color: LIGHT_GREEN, normal: front },
        // Right Face
        Vertex { position: [s, -s, s], color: LIGHT_ORANGE, normal: right }, Vertex { position: [s, -s, -s], color: LIGHT_ORANGE, normal: right }, Vertex { position: [s, s, s], color: LIGHT_ORANGE, normal: right },
        Vertex { position: [s, -s, -s], color: LIGHT_ORANGE, normal: right }, Vertex { position: [s, s, -s], color: LIGHT_ORANGE, normal: right }, Vertex { position: [s, s, s], color: LIGHT_ORANGE, normal: right },
        // Back Face
        Vertex { position: [s, -s, -s], color: LIGHT_BLUE, normal: back }, Vertex { position: [-s, -s, -s], color: LIGHT_BLUE, normal: back }, Vertex { position: [s, s, -s], color: LIGHT_BLUE, normal: back },
        Vertex { position: [-s, -s, -s], color: LIGHT_BLUE, normal: back }, Vertex { position: [-s, s, -s], color: LIGHT_BLUE, normal: back }, Vertex { position: [s, s, -s], color: LIGHT_BLUE, normal: back },
        // Left Face
        Vertex { position: [-s, -s, -s], color: LIGHT_YELLOW, normal: left }, Vertex { position: [-s, -s, s], color: LIGHT_YELLOW, normal: left }, Vertex { position: [-s, s, -s], color: LIGHT_YELLOW, normal: left },
        Vertex { position: [-s, -s, s], color: LIGHT_YELLOW, normal: left }, Vertex { position: [-s, s, s], color: LIGHT_YELLOW, normal: left }, Vertex { position: [-s, s, -s], color: LIGHT_YELLOW, normal: left },
        // Bottom Face
        Vertex { position: [-s, -s, s], color: DARK_GREY, normal: down }, Vertex { position: [-s, -s, -s], color: DARK_GREY, normal: down }, Vertex { position: [s, -s, s], color: DARK_GREY, normal: down },
        Vertex { position: [-s, -s, -s], color: DARK_GREY, normal: down }, Vertex { position: [s, -s, -s], color: DARK_GREY, normal: down }, Vertex { position: [s, -s, s], color: DARK_GREY, normal: down },
    ]
}