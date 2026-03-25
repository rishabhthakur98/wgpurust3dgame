pub mod config;
use glam::{Vec2, Vec3};
use crate::world::AABB;

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self { 
            pos: Vec3::new(0.0, 10.0, 0.0), 
            yaw: 0.0, 
            pitch: 0.5, 
            distance: config::DEFAULT_DISTANCE 
        }
    }
    
    pub fn process_mouse(&mut self, dx: f32, dy: f32, is_freeform: bool) {
        self.yaw -= dx * config::SENSITIVITY;
        self.pitch += dy * config::SENSITIVITY;

        if is_freeform {
            self.pitch = self.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.05, std::f32::consts::FRAC_PI_2 - 0.05);
        } else {
            self.pitch = self.pitch.clamp(config::PITCH_MIN, config::PITCH_MAX);
        }
    }

    pub fn get_offset(&self, distance: f32) -> Vec3 {
        Vec3::new(
            self.yaw.sin() * self.pitch.cos() * distance,
            self.pitch.sin() * distance,
            self.yaw.cos() * self.pitch.cos() * distance,
        )
    }

    pub fn update_freeform(&mut self, dt: f32, input_dir: Vec2, speed: f32) {
        let forward = Vec3::new(
            -self.yaw.sin() * self.pitch.cos(),
            -self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos()
        ).normalize();
        
        let right = forward.cross(Vec3::Y).normalize();
        
        let move_dir = forward * input_dir.y + right * input_dir.x;
        if move_dir.length_squared() > 0.0 {
            self.pos += move_dir.normalize() * speed * dt;
        }
    }

    pub fn update(&mut self, dt: f32, player_pos: Vec3, colliders: &[AABB]) {
        let steps = 20;
        let step_size = (config::DEFAULT_DISTANCE - config::MIN_DISTANCE) / steps as f32;
        let mut target_dist = config::MIN_DISTANCE;

        for i in 0..=steps {
            let test_dist = config::MIN_DISTANCE + step_size * (i as f32);
            let cam_pos = player_pos + self.get_offset(test_dist);

            let mut colliding = cam_pos.y < 0.5;

            if !colliding {
                for c in colliders {
                    let r = 0.5; 
                    if cam_pos.x + r > c.min_x && cam_pos.x - r < c.max_x &&
                       cam_pos.y + r > c.min_y && cam_pos.y - r < c.max_y &&
                       cam_pos.z + r > c.min_z && cam_pos.z - r < c.max_z {
                        colliding = true;
                        break;
                    }
                }
            }

            if colliding {
                break; 
            } else {
                target_dist = test_dist; 
            }
        }

        let speed = if target_dist < self.distance { config::ZOOM_IN_SPEED } else { config::ZOOM_OUT_SPEED };
        self.distance += (target_dist - self.distance) * speed * dt;
    }
}