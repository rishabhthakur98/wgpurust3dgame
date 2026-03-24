pub mod high_building;
pub mod pyramid;

pub struct AABB {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

pub fn get_colliders() -> Vec<AABB> {
    vec![
        high_building::get_aabb(),
        pyramid::get_aabb(),
    ]
}