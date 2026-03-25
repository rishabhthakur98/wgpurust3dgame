struct PointLight {
    position: vec4<f32>,
    color: vec4<f32>,
};

struct UniformData {
    mvp_matrix: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
    light_mvp_matrix: mat4x4<f32>, // The Sun's Camera Matrix
    sun_dir: vec4<f32>,
    sun_color: vec4<f32>,
    ambient_color: vec4<f32>,
    point_lights: array<PointLight, 2>,
};
@group(0) @binding(0) var<uniform> ubo: UniformData;

struct VertexInput {
    @location(0) position: vec3<f32>,
    // We don't care about normals, colors, or UVs for the shadow pass!
};

@vertex
fn vs_main(model: VertexInput) -> @builtin(position) vec4<f32> {
    // Calculate the position of the vertex from the SUN's perspective
    return ubo.light_mvp_matrix * ubo.model_matrix * vec4<f32>(model.position, 1.0);
}