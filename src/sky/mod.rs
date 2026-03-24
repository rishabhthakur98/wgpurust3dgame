pub mod config;
pub mod colors;

use crate::render::Vertex;

pub fn create_vertices() -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let phi = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt()); 

    for i in 0..config::STAR_COUNT {
        let y = 1.0 - (i as f32 / (config::STAR_COUNT as f32 - 1.0)) * 2.0; 
        let current_radius = (1.0 - y * y).sqrt() * config::STAR_RADIUS; 
        let theta = phi * i as f32; 
        
        let x = theta.cos() * current_radius;
        let z = theta.sin() * current_radius;
        
        vertices.push(Vertex { position: [x, y * config::STAR_RADIUS, z], color: colors::WHITE, normal: [0.0, 0.0, 0.0] });
    }
    vertices
}