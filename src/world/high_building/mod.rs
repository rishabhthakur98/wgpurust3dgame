pub mod colors;

use crate::render::Vertex;
use crate::world::AABB;

pub const POS_X: f32 = 400.0; 
pub const POS_Z: f32 = 800.0; 
pub const BREADTH: f32 = 10.0;
pub const LENGTH: f32 = 20.0;
pub const HEIGHT: f32 = 100.0;

pub fn get_aabb() -> AABB {
    AABB {
        min_x: POS_X - BREADTH / 2.0, max_x: POS_X + BREADTH / 2.0,
        min_y: 0.0, max_y: HEIGHT,
        min_z: POS_Z - LENGTH / 2.0, max_z: POS_Z + LENGTH / 2.0,
    }
}

pub fn create_vertices() -> Vec<Vertex> {
    let hx = BREADTH / 2.0; let hz = LENGTH / 2.0;
    let y0 = 0.0; let y1 = HEIGHT;
    
    let mut v = Vec::new();
    use colors::*;
    
    let up = [0.0, 1.0, 0.0];
    let front = [0.0, 0.0, 1.0]; let back = [0.0, 0.0, -1.0];
    let right = [1.0, 0.0, 0.0]; let left = [-1.0, 0.0, 0.0];
    
    // Front Face (+Z)
    v.extend_from_slice(&[
        Vertex { position: [POS_X - hx, y0, POS_Z + hz], color: FACE_FRONT, normal: front },
        Vertex { position: [POS_X + hx, y0, POS_Z + hz], color: FACE_FRONT, normal: front },
        Vertex { position: [POS_X - hx, y1, POS_Z + hz], color: FACE_FRONT, normal: front },
        Vertex { position: [POS_X + hx, y0, POS_Z + hz], color: FACE_FRONT, normal: front },
        Vertex { position: [POS_X + hx, y1, POS_Z + hz], color: FACE_FRONT, normal: front },
        Vertex { position: [POS_X - hx, y1, POS_Z + hz], color: FACE_FRONT, normal: front },
    ]);
    // Back Face (-Z)
    v.extend_from_slice(&[
        Vertex { position: [POS_X - hx, y0, POS_Z - hz], color: FACE_BACK, normal: back },
        Vertex { position: [POS_X - hx, y1, POS_Z - hz], color: FACE_BACK, normal: back },
        Vertex { position: [POS_X + hx, y0, POS_Z - hz], color: FACE_BACK, normal: back },
        Vertex { position: [POS_X + hx, y0, POS_Z - hz], color: FACE_BACK, normal: back },
        Vertex { position: [POS_X - hx, y1, POS_Z - hz], color: FACE_BACK, normal: back },
        Vertex { position: [POS_X + hx, y1, POS_Z - hz], color: FACE_BACK, normal: back },
    ]);
    // Left Face (-X)
    v.extend_from_slice(&[
        Vertex { position: [POS_X - hx, y0, POS_Z - hz], color: FACE_LEFT, normal: left },
        Vertex { position: [POS_X - hx, y0, POS_Z + hz], color: FACE_LEFT, normal: left },
        Vertex { position: [POS_X - hx, y1, POS_Z - hz], color: FACE_LEFT, normal: left },
        Vertex { position: [POS_X - hx, y0, POS_Z + hz], color: FACE_LEFT, normal: left },
        Vertex { position: [POS_X - hx, y1, POS_Z + hz], color: FACE_LEFT, normal: left },
        Vertex { position: [POS_X - hx, y1, POS_Z - hz], color: FACE_LEFT, normal: left },
    ]);
    // Right Face (+X)
    v.extend_from_slice(&[
        Vertex { position: [POS_X + hx, y0, POS_Z - hz], color: FACE_RIGHT, normal: right },
        Vertex { position: [POS_X + hx, y1, POS_Z - hz], color: FACE_RIGHT, normal: right },
        Vertex { position: [POS_X + hx, y0, POS_Z + hz], color: FACE_RIGHT, normal: right },
        Vertex { position: [POS_X + hx, y0, POS_Z + hz], color: FACE_RIGHT, normal: right },
        Vertex { position: [POS_X + hx, y1, POS_Z - hz], color: FACE_RIGHT, normal: right },
        Vertex { position: [POS_X + hx, y1, POS_Z + hz], color: FACE_RIGHT, normal: right },
    ]);
    // Top Face
    v.extend_from_slice(&[
        Vertex { position: [POS_X - hx, y1, POS_Z - hz], color: FACE_TOP, normal: up },
        Vertex { position: [POS_X - hx, y1, POS_Z + hz], color: FACE_TOP, normal: up },
        Vertex { position: [POS_X + hx, y1, POS_Z - hz], color: FACE_TOP, normal: up },
        Vertex { position: [POS_X - hx, y1, POS_Z + hz], color: FACE_TOP, normal: up },
        Vertex { position: [POS_X + hx, y1, POS_Z + hz], color: FACE_TOP, normal: up },
        Vertex { position: [POS_X + hx, y1, POS_Z - hz], color: FACE_TOP, normal: up },
    ]);
    v
}