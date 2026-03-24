struct PointLight {
    position: vec4<f32>,
    color: vec4<f32>,
};

struct UniformData {
    mvp_matrix: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
    sun_dir: vec4<f32>,
    sun_color: vec4<f32>,
    ambient_color: vec4<f32>,
    point_lights: array<PointLight, 2>,
};
@group(0) @binding(0) var<uniform> ubo: UniformData;

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>, @location(1) color: vec3<f32>, @location(2) normal: vec3<f32>, @location(3) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>, 
    @location(2) tex_coords: vec2<f32>,
    @location(3) world_pos: vec3<f32>, // NEW: Exact position in the 3D world
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = ubo.mvp_matrix * vec4<f32>(model.position, 1.0);
    out.world_pos = (ubo.model_matrix * vec4<f32>(model.position, 1.0)).xyz; // Calculate World Pos
    out.color = model.color;
    out.normal = (ubo.model_matrix * vec4<f32>(model.normal, 0.0)).xyz;
    out.tex_coords = model.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (length(in.normal) < 0.1) {
        return vec4<f32>(in.color, 1.0); // Emissive objects
    }

    let object_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let normal = normalize(in.normal);
    
    // 1. Calculate Sun Light
    let sun_dir = normalize(ubo.sun_dir.xyz);
    let sun_diffuse = max(dot(normal, sun_dir), 0.0) * ubo.sun_color.w;
    var lighting = ubo.ambient_color.xyz + (ubo.sun_color.xyz * sun_diffuse);

    // 2. Calculate Point Lights (Street Lamps)
    for (var i = 0u; i < 2u; i = i + 1u) {
        let pl = ubo.point_lights[i];
        let light_vec = pl.position.xyz - in.world_pos;
        let distance = length(light_vec);
        let dir = light_vec / distance;
        
        // Physics attenuation math: 1 / (1 + linear*d + quadratic*d^2)
        let attenuation = 1.0 / (1.0 + 0.045 * distance + 0.0075 * (distance * distance));
        
        let diffuse = max(dot(normal, dir), 0.0);
        lighting += pl.color.xyz * (diffuse * pl.color.w * attenuation);
    }
    
    let final_color = object_color.xyz * in.color * lighting;
    return vec4<f32>(final_color, object_color.a); 
}