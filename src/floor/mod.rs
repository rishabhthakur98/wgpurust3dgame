pub mod config;
pub mod colors;

use crate::render::Vertex;

pub fn create_vertices() -> Vec<Vertex> {
    let hw = config::SURFACE_WIDTH / 2.0;
    let hl = config::SURFACE_LENGTH / 2.0;
    vec![
        Vertex { position: [-hw, 0.0, -hl], color: colors::LIGHT_GREY },
        Vertex { position: [-hw, 0.0,  hl], color: colors::LIGHT_GREY },
        Vertex { position: [ hw, 0.0, -hl], color: colors::LIGHT_GREY },
        Vertex { position: [-hw, 0.0,  hl], color: colors::LIGHT_GREY },
        Vertex { position: [ hw, 0.0,  hl], color: colors::LIGHT_GREY },
        Vertex { position: [ hw, 0.0, -hl], color: colors::LIGHT_GREY },
    ]
}