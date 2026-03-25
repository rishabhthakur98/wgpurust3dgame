use crate::render::Vertex;
use crate::world::AABB;
use glam::{Mat4, Quat, Vec3};

pub const TEXTURE_BYTES: &[u8] = include_bytes!("building.png");

pub struct Building01 {
    pub x: f32, pub y: f32, pub z: f32,
    pub width: f32, pub length: f32, pub height: f32,
    pub rot_x: f32, pub rot_y: f32, pub rot_z: f32,
    pub scale: f32,
}

impl Building01 {
    pub fn new(x: f32, y: f32, z: f32, width: f32, length: f32, height: f32, rot_x: f32, rot_y: f32, rot_z: f32, scale: f32) -> Self {
        Self { x, y, z, width, length, height, rot_x, rot_y, rot_z, scale }
    }

    pub fn get_aabb(&self) -> AABB {
        // Apply scaling to the bounding box!
        let hw = (self.width * self.scale) / 2.0; 
        let hl = (self.length * self.scale) / 2.0;
        let h_scaled = self.height * self.scale;
        
        AABB {
            min_x: self.x - hw, max_x: self.x + hw,
            min_y: self.y, max_y: self.y + h_scaled,
            min_z: self.z - hl, max_z: self.z + hl,
        }
    }

    pub fn create_vertices(&self) -> Vec<Vertex> {
        let mut v = Vec::new();
        let hw = self.width / 2.0; let hl = self.length / 2.0;
        let y0 = 0.0; let y1 = self.height;
        let c = [1.0, 1.0, 1.0]; 
        let up = [0.0, 1.0, 0.0]; let front = [0.0, 0.0, 1.0]; let back = [0.0, 0.0, -1.0]; 
        let right = [1.0, 0.0, 0.0]; let left = [-1.0, 0.0, 0.0];
        let ty = self.height / 10.0;

        v.extend_from_slice(&[
            Vertex { position: [-hw, y0, hl], color: c, normal: front, tex_coords: [0.0, ty] }, Vertex { position: [hw, y0, hl], color: c, normal: front, tex_coords: [1.0, ty] }, Vertex { position: [-hw, y1, hl], color: c, normal: front, tex_coords: [0.0, 0.0] },
            Vertex { position: [hw, y0, hl], color: c, normal: front, tex_coords: [1.0, ty] }, Vertex { position: [hw, y1, hl], color: c, normal: front, tex_coords: [1.0, 0.0] }, Vertex { position: [-hw, y1, hl], color: c, normal: front, tex_coords: [0.0, 0.0] },
            Vertex { position: [-hw, y0, -hl], color: c, normal: back, tex_coords: [1.0, ty] }, Vertex { position: [-hw, y1, -hl], color: c, normal: back, tex_coords: [1.0, 0.0] }, Vertex { position: [hw, y0, -hl], color: c, normal: back, tex_coords: [0.0, ty] },
            Vertex { position: [hw, y0, -hl], color: c, normal: back, tex_coords: [0.0, ty] }, Vertex { position: [-hw, y1, -hl], color: c, normal: back, tex_coords: [1.0, 0.0] }, Vertex { position: [hw, y1, -hl], color: c, normal: back, tex_coords: [0.0, 0.0] },
            Vertex { position: [-hw, y0, -hl], color: c, normal: left, tex_coords: [0.0, ty] }, Vertex { position: [-hw, y0, hl], color: c, normal: left, tex_coords: [2.0, ty] }, Vertex { position: [-hw, y1, -hl], color: c, normal: left, tex_coords: [0.0, 0.0] },
            Vertex { position: [-hw, y0, hl], color: c, normal: left, tex_coords: [2.0, ty] }, Vertex { position: [-hw, y1, hl], color: c, normal: left, tex_coords: [2.0, 0.0] }, Vertex { position: [-hw, y1, -hl], color: c, normal: left, tex_coords: [0.0, 0.0] },
            Vertex { position: [hw, y0, -hl], color: c, normal: right, tex_coords: [2.0, ty] }, Vertex { position: [hw, y1, -hl], color: c, normal: right, tex_coords: [2.0, 0.0] }, Vertex { position: [hw, y0, hl], color: c, normal: right, tex_coords: [0.0, ty] },
            Vertex { position: [hw, y0, hl], color: c, normal: right, tex_coords: [0.0, ty] }, Vertex { position: [hw, y1, -hl], color: c, normal: right, tex_coords: [2.0, 0.0] }, Vertex { position: [hw, y1, hl], color: c, normal: right, tex_coords: [0.0, 0.0] },
            Vertex { position: [-hw, y1, -hl], color: c, normal: up, tex_coords: [0.0, 0.0] }, Vertex { position: [-hw, y1, hl], color: c, normal: up, tex_coords: [0.0, 2.0] }, Vertex { position: [hw, y1, -hl], color: c, normal: up, tex_coords: [1.0, 0.0] },
            Vertex { position: [-hw, y1, hl], color: c, normal: up, tex_coords: [0.0, 2.0] }, Vertex { position: [hw, y1, hl], color: c, normal: up, tex_coords: [1.0, 2.0] }, Vertex { position: [hw, y1, -hl], color: c, normal: up, tex_coords: [1.0, 0.0] },
        ]);

        let rotation = Quat::from_euler(glam::EulerRot::YXZ, self.rot_y, self.rot_x, self.rot_z);
        let transform = Mat4::from_scale_rotation_translation(Vec3::splat(self.scale), rotation, Vec3::new(self.x, self.y, self.z));
        
        for vertex in &mut v {
            vertex.position = (transform * Vec3::from(vertex.position).extend(1.0)).truncate().into();
            vertex.normal = (transform * Vec3::from(vertex.normal).extend(0.0)).truncate().normalize().into();
        }
        v
    }
}
