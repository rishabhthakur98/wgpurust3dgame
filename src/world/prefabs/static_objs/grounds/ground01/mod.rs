use crate::render::Vertex;
use glam::{Mat4, Quat, Vec3};

pub const TEXTURE_BYTES: &[u8] = include_bytes!("floor.png");

pub struct Ground01 {
    pub x: f32, pub y: f32, pub z: f32,
    pub width: f32, pub length: f32, pub uv_scale: f32,
    pub rot_x: f32, pub rot_y: f32, pub rot_z: f32,
    pub scale: f32,
}

impl Ground01 {
    pub fn new(x: f32, y: f32, z: f32, width: f32, length: f32, uv_scale: f32, rot_x: f32, rot_y: f32, rot_z: f32, scale: f32) -> Self {
        Self { x, y, z, width, length, uv_scale, rot_x, rot_y, rot_z, scale }
    }

    pub fn create_vertices(&self) -> Vec<Vertex> {
        let mut v = Vec::new();
        let hw = self.width / 2.0; let hl = self.length / 2.0;
        let normal = [0.0, 1.0, 0.0]; let c = [0.8, 0.8, 0.8]; 
        let uw = self.width / self.uv_scale; let ul = self.length / self.uv_scale;

        v.extend_from_slice(&[
            Vertex { position: [-hw, 0.0, -hl], color: c, normal, tex_coords: [0.0, 0.0] },
            Vertex { position: [-hw, 0.0,  hl], color: c, normal, tex_coords: [0.0, ul] },
            Vertex { position: [ hw, 0.0, -hl], color: c, normal, tex_coords: [uw, 0.0] },
            Vertex { position: [-hw, 0.0,  hl], color: c, normal, tex_coords: [0.0, ul] },
            Vertex { position: [ hw, 0.0,  hl], color: c, normal, tex_coords: [uw, ul] },
            Vertex { position: [ hw, 0.0, -hl], color: c, normal, tex_coords: [uw, 0.0] },
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
