struct UniformData {
    mvp_matrix: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
    light_dir: vec4<f32>,
    light_color: vec4<f32>,
};
@group(0) @binding(0) var<uniform> ubo: UniformData;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>, 
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = ubo.mvp_matrix * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    
    // Rotate the normal using the model matrix so it points the correct way when the object rotates
    out.normal = (ubo.model_matrix * vec4<f32>(model.normal, 0.0)).xyz;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Emissive objects (like stars) have 0,0,0 normals
    if (length(in.normal) < 0.1) {
        return vec4<f32>(in.color, 1.0);
    }

    let normal = normalize(in.normal);
    let light_dir = normalize(ubo.light_dir.xyz);
    
    let ambient_strength = ubo.light_color.w;
    let diffuse_strength = max(dot(normal, light_dir), 0.0);
    
    let lighting = ambient_strength + (diffuse_strength * (1.0 - ambient_strength));
    let final_color = in.color * ubo.light_color.xyz * lighting;
    
    return vec4<f32>(final_color, 1.0); 
}