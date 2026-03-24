pub mod colors;

use crate::render::Vertex;
use crate::world::AABB;

pub const POS_X: f32 = 300.0;
pub const POS_Z: f32 = 300.0;
pub const BASE_SIZE: f32 = 20.0;
pub const HEIGHT: f32 = 30.0;

pub fn get_aabb() -> AABB {
    let hs = BASE_SIZE / 2.0;
    AABB {
        min_x: POS_X - hs, max_x: POS_X + hs,
        min_z: POS_Z - hs, max_z: POS_Z + hs,
    }
}

pub fn create_vertices() -> Vec<Vertex> {
    let hs = BASE_SIZE / 2.0;
    use colors::*;

    let tip = [POS_X, HEIGHT, POS_Z];
    let a = [POS_X - hs, 0.0, POS_Z - hs];
    let b = [POS_X + hs, 0.0, POS_Z - hs];
    let c = [POS_X + hs, 0.0, POS_Z + hs];
    let d = [POS_X - hs, 0.0, POS_Z + hs];

    vec![
        // Face 1
        Vertex { position: tip, color: FACE_1 }, Vertex { position: a, color: FACE_1 }, Vertex { position: b, color: FACE_1 },
        // Face 2
        Vertex { position: tip, color: FACE_2 }, Vertex { position: b, color: FACE_2 }, Vertex { position: c, color: FACE_2 },
        // Face 3
        Vertex { position: tip, color: FACE_3 }, Vertex { position: c, color: FACE_3 }, Vertex { position: d, color: FACE_3 },
        // Face 4
        Vertex { position: tip, color: FACE_4 }, Vertex { position: d, color: FACE_4 }, Vertex { position: a, color: FACE_4 },
    ]
}