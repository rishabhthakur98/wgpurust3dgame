pub mod prefabs;

use crate::render::Vertex;
use prefabs::static_objs::grounds::ground01::Ground01;
use prefabs::static_objs::buildings::building01::Building01;
use prefabs::static_objs::buildings::building02::Building02;
use prefabs::static_objs::streetlights::streetlight01::Streetlight01;

pub struct AABB {
    pub min_x: f32, pub max_x: f32,
    pub min_y: f32, pub max_y: f32,
    pub min_z: f32, pub max_z: f32,
}

pub struct WorldState {
    pub grounds: Vec<Ground01>,
    pub building01s: Vec<Building01>,
    pub building02s: Vec<Building02>,
    pub streetlights: Vec<Streetlight01>,
}

impl WorldState {
    pub fn new() -> Self {
        let mut my_streetlights = Vec::new();
        
        // Loop to create 100 streetlights total (50 pairs)
        for i in 0..50 {
            let z_position = 100.0 + (i as f32 * 10.0);
            
            // Left pole (x: 347.5). Rotations: (0,0,0). Scale: 1.0
            my_streetlights.push(Streetlight01::new(347.5, 0.0, z_position, 0.0, 0.0, 0.0, 1.0));
            // Right pole (x: 352.5). Rotations: (0, PI, 0). Scale: 1.0
            my_streetlights.push(Streetlight01::new(352.5, 0.0, z_position, 0.0, std::f32::consts::PI, 0.0, 1.0));
        }

        Self {
            // New Ground01 signature: x, y, z, width, length, uv_scale, rot_x, rot_y, rot_z, scale
            grounds: vec![Ground01::new(0.0, 0.0, 0.0, 2000.0, 2000.0, 10.0, 0.0, 0.0, 0.0, 1.0)],
            
            building01s: vec![
                // New Building01 signature: x, y, z, width, length, height, rot_x, rot_y, rot_z, scale
                // Normal standing building
                Building01::new(400.0, 0.0, 800.0, 10.0, 20.0, 100.0, 0.0, 0.0, 0.0, 1.0),
                
                // Horizontal building (Roll/rot_z = 1.57 radians)
                Building01::new(450.0, 10.0, 800.0, 10.0, 20.0, 100.0, 0.0, 0.0, 1.57, 1.0), 
            ],
            
            building02s: vec![
                // New Building02 signature: x, y, z, base_width, height, rot_x, rot_y, rot_z, scale
                // Normal Pyramid
                Building02::new(300.0, 0.0, 300.0, 50.0, 40.0, 0.0, 0.0, 0.0, 1.0),
                
                // Tumbled Pyramid (Pitch/rot_x = PI radians)
                Building02::new(380.0, 40.0, 300.0, 50.0, 40.0, std::f32::consts::PI, 0.0, 0.0, 1.0),
                
                // Example of a tiny half-sized pyramid using scale!
                Building02::new(340.0, 0.0, 250.0, 50.0, 40.0, 0.0, 0.0, 0.0, 0.5),
            ],
            
            streetlights: my_streetlights,
        }
    }

    pub fn get_colliders(&self) -> Vec<AABB> {
        let mut colliders = Vec::new();
        for b in &self.building01s { colliders.push(b.get_aabb()); }
        for p in &self.building02s { colliders.push(p.get_aabb()); }
        for l in &self.streetlights { colliders.push(l.get_aabb()); }
        colliders
    }

    pub fn get_ground_vertices(&self) -> Vec<Vertex> { self.grounds.iter().flat_map(|g| g.create_vertices()).collect() }
    pub fn get_building01_vertices(&self) -> Vec<Vertex> { self.building01s.iter().flat_map(|b| b.create_vertices()).collect() }
    pub fn get_building02_vertices(&self) -> Vec<Vertex> { self.building02s.iter().flat_map(|p| p.create_vertices()).collect() }
    pub fn get_streetlight_vertices(&self) -> Vec<Vertex> { self.streetlights.iter().flat_map(|l| l.create_vertices()).collect() }
}
