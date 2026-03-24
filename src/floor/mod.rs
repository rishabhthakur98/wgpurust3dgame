pub mod config;
use crate::core::colors::LIGHT_GREY;
use crate::render::Vertex;

pub fn create_vertices() -> Vec<Vertex> {
    let s = config::SURFACE_SIZE / 2.0; 
    vec![
        Vertex { position: [-s, 0.0, -s], color: LIGHT_GREY }, Vertex { position: [-s, 0.0,  s], color: LIGHT_GREY }, Vertex { position: [ s, 0.0, -s], color: LIGHT_GREY },
        Vertex { position: [-s, 0.0,  s], color: LIGHT_GREY }, Vertex { position: [ s, 0.0,  s], color: LIGHT_GREY }, Vertex { position: [ s, 0.0, -s], color: LIGHT_GREY },
    ]
}