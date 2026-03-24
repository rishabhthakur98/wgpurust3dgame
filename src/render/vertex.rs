use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3], pub color: [f32; 3], pub normal: [f32; 3], pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                wgpu::VertexAttribute { offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, shader_location: 1, format: wgpu::VertexFormat::Float32x3 },
                wgpu::VertexAttribute { offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress, shader_location: 2, format: wgpu::VertexFormat::Float32x3 },
                wgpu::VertexAttribute { offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress, shader_location: 3, format: wgpu::VertexFormat::Float32x2 } 
            ],
        }
    }
}

// Struct to hold street light data
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct PointLight {
    pub position: [f32; 4], // x, y, z, padding
    pub color: [f32; 4],    // r, g, b, intensity
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct UniformData { 
    pub mvp_matrix: [[f32; 4]; 4],
    pub model_matrix: [[f32; 4]; 4], 
    pub sun_dir: [f32; 4],         
    pub sun_color: [f32; 4], // Intensity in .w
    pub ambient_color: [f32; 4],
    pub point_lights: [PointLight; 2], // Pass our 2 street lamps
}