pub mod config;
use glam::Vec3;
use crate::world::AABB;

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

    // Helper to calculate camera offset position based on a distance
    pub fn get_offset(&self, distance: f32) -> Vec3 {
        Vec3::new(
            self.yaw.sin() * self.pitch.cos() * distance,
            self.pitch.sin() * distance,
            self.yaw.cos() * self.pitch.cos() * distance,
        )
    }

    // Calculate dynamic distance to prevent clipping into objects or the floor
    pub fn update(&mut self, player_pos: Vec3, colliders: &[AABB]) {
        let steps = 20;
        let step_size = (config::DEFAULT_DISTANCE - config::MIN_DISTANCE) / steps as f32;
        
        let mut best_dist = config::MIN_DISTANCE;

        // March outward from MIN to DEFAULT. Stop at the first collision.
        for i in 0..=steps {
            let test_dist = config::MIN_DISTANCE + step_size * (i as f32);
            let cam_pos = player_pos + self.get_offset(test_dist);

            // Floor check (0.5 padding so the camera doesn't visually cut into the ground)
            let mut colliding = cam_pos.y < 0.5;

            // Object check
            if !colliding {
                for c in colliders {
                    let r = 0.5; // padding/radius around the camera
                    if cam_pos.x + r > c.min_x && cam_pos.x - r < c.max_x &&
                       cam_pos.y + r > c.min_y && cam_pos.y - r < c.max_y &&
                       cam_pos.z + r > c.min_z && cam_pos.z - r < c.max_z {
                        colliding = true;
                        break;
                    }
                }
            }

            if colliding {
                break; // Hit something! Stop expanding distance.
            } else {
                best_dist = test_dist; // Safe distance, keep it and try expanding more.
            }
        }

        self.distance = best_dist;
    }
}