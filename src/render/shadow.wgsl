struct PointLight {
    position: vec4<f32>,
    color: vec4<f32>,
};

struct UniformData {
    mvp_matrix: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
    light_mvp_matrix: mat4x4<f32>,
    sun_dir: vec4<f32>,
    sun_color: vec4<f32>,
    ambient_color: vec4<f32>,
    point_lights: array<PointLight, 2>,
    flashlight_pos: vec4<f32>,
    flashlight_dir: vec4<f32>,
    flashlight_color: vec4<f32>,
    sky_mvp_matrix: mat4x4<f32>,
    sky_zenith: vec4<f32>,
    sky_horizon: vec4<f32>,
    sky_night: vec4<f32>,
};
@group(0) @binding(0) var<uniform> ubo: UniformData;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> @builtin(position) vec4<f32> {
    return ubo.light_mvp_matrix * ubo.model_matrix * vec4<f32>(model.position, 1.0);
}