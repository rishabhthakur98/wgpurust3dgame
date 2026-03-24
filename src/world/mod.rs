pub mod high_building;
pub mod pyramid;
pub mod street_light; // Expose the new street light module

pub struct AABB {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
}

pub fn get_colliders() -> Vec<AABB> {
    let mut colliders = vec![
        high_building::get_aabb(),
        pyramid::get_aabb(),
    ];
    // Add both street poles to the world collision detection
    colliders.extend(street_light::get_colliders());
    
    colliders
}