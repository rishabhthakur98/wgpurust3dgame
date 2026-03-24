pub mod colors;

use crate::render::Vertex;
use crate::world::AABB;
use glam::Vec3;

pub const POS_X: f32 = 300.0;
pub const POS_Z: f32 = 300.0;
pub const BASE_SIZE: f32 = 20.0;
pub const HEIGHT: f32 = 30.0;

pub fn get_aabb() -> AABB {
    let hs = BASE_SIZE / 2.0;
    AABB {
        min_x: POS_X - hs, max_x: POS_X + hs,
        min_y: 0.0, max_y: HEIGHT,
        min_z: POS_Z - hs, max_z: POS_Z + hs,
    }
}

fn calc_normal(p1: [f32; 3], p2: [f32; 3], p3: [f32; 3]) -> [f32; 3] {
    let v1 = Vec3::from(p1); let v2 = Vec3::from(p2); let v3 = Vec3::from(p3);
    let mut normal = (v2 - v1).cross(v3 - v1).normalize();
    if normal.y < 0.0 { normal = -normal; }
    normal.into()
}

pub fn create_vertices() -> Vec<Vertex> {
    let hs = BASE_SIZE / 2.0;
    use colors::*;

    let tip = [POS_X, HEIGHT, POS_Z];
    let a = [POS_X - hs, 0.0, POS_Z - hs];
    let b = [POS_X + hs, 0.0, POS_Z - hs];
    let c = [POS_X + hs, 0.0, POS_Z + hs];
    let d = [POS_X - hs, 0.0, POS_Z + hs];

    let n1 = calc_normal(tip, a, b);
    let n2 = calc_normal(tip, b, c);
    let n3 = calc_normal(tip, c, d);
    let n4 = calc_normal(tip, d, a);

    vec![
        Vertex { position: tip, color: FACE_1, normal: n1 }, Vertex { position: a, color: FACE_1, normal: n1 }, Vertex { position: b, color: FACE_1, normal: n1 },
        Vertex { position: tip, color: FACE_2, normal: n2 }, Vertex { position: b, color: FACE_2, normal: n2 }, Vertex { position: c, color: FACE_2, normal: n2 },
        Vertex { position: tip, color: FACE_3, normal: n3 }, Vertex { position: c, color: FACE_3, normal: n3 }, Vertex { position: d, color: FACE_3, normal: n3 },
        Vertex { position: tip, color: FACE_4, normal: n4 }, Vertex { position: d, color: FACE_4, normal: n4 }, Vertex { position: a, color: FACE_4, normal: n4 },
    ]
}