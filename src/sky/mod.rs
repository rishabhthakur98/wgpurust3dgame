pub mod config;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkyboxVertex {
    pub position: [f32; 3],
}

impl SkyboxVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SkyboxVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
            ],
        }
    }
}

pub fn create_skybox_cube() -> Vec<SkyboxVertex> {
    // A standard 1x1x1 cube centered at origin
    let s = 1.0;
    let pos = [
        // Front
        [-s, -s,  s], [ s, -s,  s], [ s,  s,  s], [-s,  s,  s],
        // Back
        [-s, -s, -s], [-s,  s, -s], [ s,  s, -s], [ s, -s, -s],
        // Top
        [-s,  s, -s], [-s,  s,  s], [ s,  s,  s], [ s,  s, -s],
        // Bottom
        [-s, -s, -s], [ s, -s, -s], [ s, -s,  s], [-s, -s,  s],
        // Right
        [ s, -s, -s], [ s,  s, -s], [ s,  s,  s], [ s, -s,  s],
        // Left
        [-s, -s, -s], [-s, -s,  s], [-s,  s,  s], [-s,  s, -s],
    ];

    let indices = [
        0, 1, 2, 2, 3, 0,       // Front
        4, 5, 6, 6, 7, 4,       // Back
        8, 9, 10, 10, 11, 8,    // Top
        12, 13, 14, 14, 15, 12, // Bottom
        16, 17, 18, 18, 19, 16, // Right
        20, 21, 22, 22, 23, 20, // Left
    ];

    indices.iter().map(|&i| SkyboxVertex { position: pos[i] }).collect()
}