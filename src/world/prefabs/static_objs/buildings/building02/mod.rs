use crate::render::Vertex;
use crate::world::AABB;
use glam::{Mat4, Quat, Vec3};

pub const TEXTURE_BYTES: &[u8] = include_bytes!("pyramid.png");

pub struct Building02 {
    pub x: f32, pub y: f32, pub z: f32,
    pub base_width: f32, pub height: f32,
    pub rot_x: f32, pub rot_y: f32, pub rot_z: f32,
    pub scale: f32,
}

impl Building02 {
    pub fn new(x: f32, y: f32, z: f32, base_width: f32, height: f32, rot_x: f32, rot_y: f32, rot_z: f32, scale: f32) -> Self {
        Self { x, y, z, base_width, height, rot_x, rot_y, rot_z, scale }
    }

    pub fn get_aabb(&self) -> AABB {
        let hw = (self.base_width * self.scale) / 2.0;
        let h_scaled = self.height * self.scale;
        AABB {
            min_x: self.x - hw, max_x: self.x + hw,
            min_y: self.y, max_y: self.y + h_scaled,
            min_z: self.z - hw, max_z: self.z + hw,
        }
    }

    pub fn create_vertices(&self) -> Vec<Vertex> {
        let mut v = Vec::new();
        let hw = self.base_width / 2.0; let c = [1.0, 1.0, 1.0]; 
        
        let top = [0.0, self.height, 0.0];
        let bl = [-hw, 0.0,  hw]; let br = [ hw, 0.0,  hw]; let tr = [ hw, 0.0, -hw]; let tl = [-hw, 0.0, -hw]; 

        let n_front = Vec3::new(0.0, hw, self.height).normalize().into();
        let n_back = Vec3::new(0.0, hw, -self.height).normalize().into();
        let n_left = Vec3::new(-self.height, hw, 0.0).normalize().into();
        let n_right = Vec3::new(self.height, hw, 0.0).normalize().into();

        v.extend_from_slice(&[
            Vertex { position: bl, color: c, normal: n_front, tex_coords: [0.0, 1.0] }, Vertex { position: br, color: c, normal: n_front, tex_coords: [1.0, 1.0] }, Vertex { position: top, color: c, normal: n_front, tex_coords: [0.5, 0.0] },
            Vertex { position: br, color: c, normal: n_back, tex_coords: [0.0, 1.0] }, Vertex { position: tr, color: c, normal: n_back, tex_coords: [1.0, 1.0] }, Vertex { position: top, color: c, normal: n_back, tex_coords: [0.5, 0.0] },
            Vertex { position: tr, color: c, normal: n_right, tex_coords: [0.0, 1.0] }, Vertex { position: tl, color: c, normal: n_right, tex_coords: [1.0, 1.0] }, Vertex { position: top, color: c, normal: n_right, tex_coords: [0.5, 0.0] },
            Vertex { position: tl, color: c, normal: n_left, tex_coords: [0.0, 1.0] }, Vertex { position: bl, color: c, normal: n_left, tex_coords: [1.0, 1.0] }, Vertex { position: top, color: c, normal: n_left, tex_coords: [0.5, 0.0] },
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
