use crate::render::vertex::Vertex;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min_x: f32, pub max_x: f32,
    pub min_y: f32, pub max_y: f32,
    pub min_z: f32, pub max_z: f32,
}

pub struct WorldState {
    pub ground_vertices: Vec<Vertex>,
}

impl WorldState {
    pub fn new() -> Self {
        // The paths to check for your models
        let glb_path = "src/world/world01/glb/grounds/ground01/ground01.glb";
        let gltf_path = "src/world/world01/gltf/grounds/ground01/ground01.gltf";

        // Try GLB first, then GLTF. If neither exist, spawn a fallback flat plane.
        let vertices = if Path::new(glb_path).exists() {
            println!("Loading GLB model: {}", glb_path);
            Self::load_model(glb_path)
        } else if Path::new(gltf_path).exists() {
            println!("Loading GLTF model: {}", gltf_path);
            Self::load_model(gltf_path)
        } else {
            println!("Warning: No 3D model found. Spawning default green plane.");
            Self::create_fallback_plane()
        };

        Self { ground_vertices: vertices }
    }

    // The Universal Model Loader
    fn load_model(path: &str) -> Vec<Vertex> {
        let (document, buffers, _images) = gltf::import(path).expect("Failed to parse GLTF/GLB");
        let mut vertices = Vec::new();

        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                // Extract mesh data from the binary blobs
                let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
                let normals: Vec<[f32; 3]> = reader.read_normals().map_or(
                    vec![[0.0, 1.0, 0.0]; positions.len()], // Fallback normal pointing straight up
                    |n| n.collect(),
                );
                let tex_coords: Vec<[f32; 2]> = reader.read_tex_coords(0).map_or(
                    vec![[0.0, 0.0]; positions.len()], // Fallback UVs
                    |t| t.into_f32().collect(),
                );

                // Flatten the indices into a raw triangle list so wgpu can draw it easily
                if let Some(indices) = reader.read_indices() {
                    for idx in indices.into_u32() {
                        let i = idx as usize;
                        vertices.push(Vertex {
                            position: positions[i],
                            normal: normals[i],
                            tex_coords: tex_coords[i],
                            color: [1.0, 1.0, 1.0], // Base white color
                        });
                    }
                } else {
                    for i in 0..positions.len() {
                        vertices.push(Vertex {
                            position: positions[i], normal: normals[i], tex_coords: tex_coords[i], color: [1.0, 1.0, 1.0],
                        });
                    }
                }
            }
        }
        vertices
    }

    fn create_fallback_plane() -> Vec<Vertex> {
        let s = 50.0;
        vec![
            Vertex { position: [-s, 0.0, -s], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], color: [0.4, 0.8, 0.4] },
            Vertex { position: [ s, 0.0, -s], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0], color: [0.4, 0.8, 0.4] },
            Vertex { position: [ s, 0.0,  s], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0], color: [0.4, 0.8, 0.4] },
            Vertex { position: [-s, 0.0, -s], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], color: [0.4, 0.8, 0.4] },
            Vertex { position: [ s, 0.0,  s], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0], color: [0.4, 0.8, 0.4] },
            Vertex { position: [-s, 0.0,  s], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0], color: [0.4, 0.8, 0.4] },
        ]
    }

    pub fn get_colliders(&self) -> Vec<AABB> {
        vec![] // Cleared out old colliders since buildings are gone
    }
}
