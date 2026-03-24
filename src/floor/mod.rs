pub mod config;
pub mod colors;
use crate::render::Vertex;

pub fn create_vertices() -> Vec<Vertex> {
    let hw = config::SURFACE_WIDTH / 2.0;
    let hl = config::SURFACE_LENGTH / 2.0;
    let normal = [0.0, 1.0, 0.0]; 
    let c = colors::LIGHT_GREY;

    // Tile the texture across the massive floor
    let uw = config::SURFACE_WIDTH / 10.0;
    let ul = config::SURFACE_LENGTH / 10.0;

    vec![
        Vertex { position: [-hw, 0.0, -hl], color: c, normal, tex_coords: [0.0, 0.0] },
        Vertex { position: [-hw, 0.0,  hl], color: c, normal, tex_coords: [0.0, ul] },
        Vertex { position: [ hw, 0.0, -hl], color: c, normal, tex_coords: [uw, 0.0] },
        Vertex { position: [-hw, 0.0,  hl], color: c, normal, tex_coords: [0.0, ul] },
        Vertex { position: [ hw, 0.0,  hl], color: c, normal, tex_coords: [uw, ul] },
        Vertex { position: [ hw, 0.0, -hl], color: c, normal, tex_coords: [uw, 0.0] },
    ]
}