// Vertex input and output
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] tex_coord: vec2<f32>;
    [[location(3)]] normal: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] vertex_position: vec3<f32>;  
};

// Data structures 
struct Camera {
    view_proj: mat4x4<f32>;
    view_pos: vec4<f32>;
};

struct Model {
    model: mat4x4<f32>;
    normal: mat3x3<f32>;
};

// Uniform bindings
[[group(0), binding(0)]]
var<uniform> u_camera: Camera;

[[group(1), binding(0)]]
var<uniform> u_model: Model;

// Vertex Shader
[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    
    var out: VertexOutput;
    out.tex_coord = in.tex_coord;
    out.position = u_camera.view_proj * u_model.model * vec4<f32>(in.position, 1.0);
    out.vertex_position = (u_model.model * vec4<f32>(in.position, 1.0)).xyz;
    //out.normal = u_model.normal * in.normal;
    out.normal = in.normal;
    return out;
}

// ---------------------------------------------------------------- //

[[group(2), binding(0)]]
var u_diffuse_texture: texture_2d<f32>;

[[group(2), binding(1)]]
var u_sampler: sampler;

fn get_fog(d: f32) -> f32
{
    let FogMax = 2000.0;
    let FogMin = 200.0;

    if (d >= FogMax) {
        return 1.0;
    }
    
    if (d <= FogMin) {
        return 0.0;
    }

    return 1.0 - (FogMax - d) / (FogMax - FogMin);
}

fn apply_fog(rgb: vec3<f32>, d: f32, b: f32) -> vec3<f32> {    
    let fog_amount = 1.0 - exp(-d * b);
    let fog_color = vec3<f32>(0.5, 0.6, 0.7);
    return mix(rgb, fog_color, vec3<f32>(fog_amount));
}

// Fragment Shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // Sample the texture color
    let tex = textureSample(u_diffuse_texture, u_sampler, in.tex_coord);   
    let color = pow(tex.xyz, vec3<f32>(2.2));
    let alpha = tex.w;

    // Ambient Lighting
    let ambient_strength = 0.02;
    let ambient_color = vec3<f32>(1.0) * ambient_strength; 
    
    // Sun Lighting
    let light_dir = normalize(-vec3<f32>(-0.2, -1.0, -0.3));
    let diffuse_strength = max(dot(in.normal, light_dir), 0.0);
    let diffuse_color = vec3<f32>(1.0) * diffuse_strength;
    
    let color = (ambient_color + diffuse_color) * color;
    
    // Fog
    //let d = abs(distance(u_camera.view_pos.xyz, in.vertex_position));
    //let fog_mix = apply_fog(color, d, 0.00007);
    
    // Gamma Correction
    let gamma_correction = pow(color, vec3<f32>(1.0/2.2));
    return vec4<f32>(gamma_correction, alpha);    
}
