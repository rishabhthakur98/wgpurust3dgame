use crate::render::Vertex;
use crate::world::AABB;

// We'll place them exactly halfway between the Building (X:400, Z:800) and Pyramid (X:300, Z:300)
pub const POS_X: f32 = 350.0;
pub const POS_Z: f32 = 550.0;

// Returns bounding boxes for BOTH poles
pub fn get_colliders() -> Vec<AABB> {
    let r = 0.5; // Pole radius/thickness
    vec![
        AABB { min_x: POS_X - 2.5 - r, max_x: POS_X - 2.5 + r, min_y: 0.0, max_y: 10.0, min_z: POS_Z - r, max_z: POS_Z + r }, // Left Pole
        AABB { min_x: POS_X + 2.5 - r, max_x: POS_X + 2.5 + r, min_y: 0.0, max_y: 10.0, min_z: POS_Z - r, max_z: POS_Z + r }, // Right Pole
    ]
}

// Helper to draw a rectangular prism (for the poles and lamps)
fn draw_box(v: &mut Vec<Vertex>, x: f32, y: f32, z: f32, w: f32, h: f32, d: f32) {
    let c = [1.0, 1.0, 1.0]; // Color is handled by texture now
    let uv = [0.0, 0.0]; // Basic UV mapping for metal pole
    
    // Just simple normals for a box
    let up = [0.0, 1.0, 0.0]; let down = [0.0, -1.0, 0.0];
    let front = [0.0, 0.0, 1.0]; let back = [0.0, 0.0, -1.0];
    let right = [1.0, 0.0, 0.0]; let left = [-1.0, 0.0, 0.0];

    // Front
    v.extend_from_slice(&[
        Vertex { position: [x-w, y, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x+w, y, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: front, tex_coords: uv },
        Vertex { position: [x+w, y, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x+w, y+h, z+d], color: c, normal: front, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: front, tex_coords: uv },
    ]);
    // Back
    v.extend_from_slice(&[
        Vertex { position: [x-w, y, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x+w, y, z-d], color: c, normal: back, tex_coords: uv },
        Vertex { position: [x+w, y, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: back, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: back, tex_coords: uv },
    ]);
    // Left
    v.extend_from_slice(&[
        Vertex { position: [x-w, y, z-d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y, z+d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: left, tex_coords: uv },
        Vertex { position: [x-w, y, z+d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: left, tex_coords: uv }, Vertex { position: [x-w, y+h, z-d], color: c, normal: left, tex_coords: uv },
    ]);
    // Right
    v.extend_from_slice(&[
        Vertex { position: [x+w, y, z-d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y, z+d], color: c, normal: right, tex_coords: uv },
        Vertex { position: [x+w, y, z+d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: right, tex_coords: uv }, Vertex { position: [x+w, y+h, z+d], color: c, normal: right, tex_coords: uv },
    ]);
    // Top
    v.extend_from_slice(&[
        Vertex { position: [x-w, y+h, z-d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x-w, y+h, z+d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: up, tex_coords: uv },
        Vertex { position: [x-w, y+h, z+d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x+w, y+h, z+d], color: c, normal: up, tex_coords: uv }, Vertex { position: [x+w, y+h, z-d], color: c, normal: up, tex_coords: uv },
    ]);
    // Bottom
    v.extend_from_slice(&[
        Vertex { position: [x-w, y, z-d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x+w, y, z-d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x-w, y, z+d], color: c, normal: down, tex_coords: uv },
        Vertex { position: [x+w, y, z-d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x-w, y, z+d], color: c, normal: down, tex_coords: uv }, Vertex { position: [x+w, y, z+d], color: c, normal: down, tex_coords: uv },
    ]);
}

pub fn create_vertices() -> Vec<Vertex> {
    let mut v = Vec::new();
    
    // Left Pole (Separated by 5m, so x is -2.5 from center)
    draw_box(&mut v, POS_X - 2.5, 0.0, POS_Z, 0.2, 10.0, 0.2); // Vertical Pole
    draw_box(&mut v, POS_X - 1.5, 9.8, POS_Z, 1.0, 0.2, 0.2);  // Arm reaching right

    // Right Pole (x is +2.5 from center)
    draw_box(&mut v, POS_X + 2.5, 0.0, POS_Z, 0.2, 10.0, 0.2); // Vertical Pole
    draw_box(&mut v, POS_X + 1.5, 9.8, POS_Z, 1.0, 0.2, 0.2);  // Arm reaching left

    v
}