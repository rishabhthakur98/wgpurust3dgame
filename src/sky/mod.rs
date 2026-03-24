pub mod config;
pub mod colors;

use crate::render::Vertex;

pub fn create_vertices() -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    // 1. Generate Stars
    let phi = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt()); 
    for i in 0..config::STAR_COUNT {
        let y = 1.0 - (i as f32 / (config::STAR_COUNT as f32 - 1.0)) * 2.0; 
        let current_radius = (1.0 - y * y).sqrt() * config::STAR_RADIUS; 
        let theta = phi * i as f32; 
        let x = theta.cos() * current_radius;
        let z = theta.sin() * current_radius;
        vertices.push(Vertex { position: [x, y * config::STAR_RADIUS, z], color: colors::WHITE, normal: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0] });
    }

    // 2. Generate Sun (A large cube in the sky)
    // We will place it matching our light_dir: [0.8, 1.0, 0.5] scaled out
    let sx = 800.0; let sy = 1000.0; let sz = 500.0;
    let ss = 50.0; // Sun Size
    
    // Simple 6 faces for the sun, using 0,0,0 normal so the shader makes it glow pure white
    let sun_color = [1.0, 1.0, 0.9];
    let n = [0.0, 0.0, 0.0]; 
    let uv = [0.0, 0.0];
    
    vertices.extend_from_slice(&[
        Vertex { position: [sx-ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx-ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx-ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx+ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx-ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx-ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx-ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx+ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx-ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx-ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx-ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx+ss, sy-ss, sz-ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv },
        Vertex { position: [sx+ss, sy-ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz+ss], color: sun_color, normal: n, tex_coords: uv }, Vertex { position: [sx+ss, sy+ss, sz-ss], color: sun_color, normal: n, tex_coords: uv },
    ]);

    vertices
}