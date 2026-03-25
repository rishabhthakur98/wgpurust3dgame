use crate::render::Vertex;
use crate::world::AABB;
use glam::{Mat4, Quat, Vec3};

pub const TEXTURE_BYTES: &[u8] = include_bytes!("street_light.png");

pub struct Streetlight01 {
    pub x: f32, pub y: f32, pub z: f32, 
    pub rot_x: f32, pub rot_y: f32, pub rot_z: f32,
    pub scale: f32,
}

impl Streetlight01 {
    pub fn new(x: f32, y: f32, z: f32, rot_x: f32, rot_y: f32, rot_z: f32, scale: f32) -> Self { 
        Self { x, y, z, rot_x, rot_y, rot_z, scale } 
    }

    pub fn get_aabb(&self) -> AABB {
        let r = 0.5 * self.scale; 
        let h = 10.0 * self.scale;
        AABB { min_x: self.x - r, max_x: self.x + r, min_y: self.y, max_y: self.y + h, min_z: self.z - r, max_z: self.z + r }
    }

    pub fn create_vertices(&self) -> Vec<Vertex> {
        let mut v = Vec::new();
        draw_box(&mut v, 0.0, 0.0, 0.0, 0.2, 10.0, 0.2); 
        draw_box(&mut v, 1.0, 9.8, 0.0, 1.0, 0.2, 0.2);  

        let rotation = Quat::from_euler(glam::EulerRot::YXZ, self.rot_y, self.rot_x, self.rot_z);
        let transform = Mat4::from_scale_rotation_translation(Vec3::splat(self.scale), rotation, Vec3::new(self.x, self.y, self.z));
        
        for vertex in &mut v {
            vertex.position = (transform * Vec3::from(vertex.position).extend(1.0)).truncate().into();
            vertex.normal = (transform * Vec3::from(vertex.normal).extend(0.0)).truncate().normalize().into();
        }
        v
    }
}

fn draw_box(v: &mut Vec<Vertex>, x: f32, y: f32, z: f32, w: f32, h: f32, d: f32) {
    let c = [1.0, 1.0, 1.0]; let uv = [0.0, 0.0];
    let up = [0.0, 1.0, 0.0]; let down = [0.0, -1.0, 0.0];
    let front = [0.0, 0.0, 1.0]; let back = [0.0, 0.0, -1.0];
    let right = [1.0, 0.0, 0.0]; let left = [-1.0, 0.0, 0.0];
    v.extend_from_slice(&[
        Vertex { position: [x-w, y, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x+w, y, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: front, tex_coords: uv },
        Vertex { position: [x+w, y, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x+w, y+h, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: front, tex_coords: uv },
        Vertex { position: [x-w, y, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x+w, y, z-d], color: c, normal: back, tex_coords: uv },
        Vertex { position: [x+w, y, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: back, tex_coords: uv },
        Vertex { position: [x-w, y, z-d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y, z+d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: left, tex_coords: uv },
        Vertex { position: [x-w, y, z+d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: left, tex_coords: uv },
        Vertex { position: [x+w, y, z-d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y, z+d], color: c, normal: right, tex_coords: uv },
        Vertex { position: [x+w, y, z+d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y+h, z+d], color: c, normal: right, tex_coords: uv },
        Vertex { position: [x-w, y+h, z-d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: up, tex_coords: uv },
        Vertex { position: [x-w, y+h, z+d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x+w, y+h, z+d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: up, tex_coords: uv },
        Vertex { position: [x-w, y, z-d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x+w, y, z-d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x-w, y, z+d], color: c, normal: down, tex_coords: uv },
        Vertex { position: [x+w, y, z-d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x-w, y, z+d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x+w, y, z+d], color: c, normal: down, tex_coords: uv },
    ]);
}
