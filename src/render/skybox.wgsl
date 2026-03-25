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
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_dir: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let pos = ubo.sky_mvp_matrix * vec4<f32>(model.position, 1.0);
    out.clip_position = vec4<f32>(pos.xy, pos.w, pos.w); 
    
    out.view_dir = model.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dir = normalize(in.view_dir);
    let sun_dir = normalize(ubo.sun_dir.xyz);
    let is_day = ubo.sun_color.w > 0.0;
    
    let zenith = ubo.sky_zenith.xyz;
    let horizon = ubo.sky_horizon.xyz;
    let night = ubo.sky_night.xyz;
    
    let sky_blend = clamp(dir.y, 0.0, 1.0);
    var final_color = mix(horizon, zenith, sky_blend);
    
    if (!is_day) {
        final_color = night;
    }

    if (is_day) {
        let sun_dot = dot(dir, sun_dir);
        if (sun_dot > 0.999) {
            final_color = vec3<f32>(1.0, 1.0, 1.0); 
        } else if (sun_dot > 0.995) {
            let glow = (sun_dot - 0.995) / (0.999 - 0.995);
            final_color += ubo.sun_color.xyz * glow * 1.5;
        }
    }

    return vec4<f32>(final_color, 1.0);
}