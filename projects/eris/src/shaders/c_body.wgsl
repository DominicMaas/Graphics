//#include noise/noise32.wgsl

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
    [[location(3)]] raw_position: vec3<f32>;
    [[location(4)]] temp_k: f32;
    [[location(5)]] atmosphere_density: f32;
};

// Data structures 
struct Camera {
    view_proj: mat4x4<f32>;
};

struct BodyDetails {
    model: mat4x4<f32>;
    normal: mat3x3<f32>;
    temp_k: f32;
    atmosphere_density: f32;
};

struct Light {
    position: vec4<f32>;
    color: vec4<f32>;
};

// Uniform bindings
[[group(0), binding(0)]]
var u_diffuse_texture: texture_2d<f32>;

[[group(0), binding(1)]]
var u_sampler: sampler;

[[group(1), binding(0)]]
var<uniform> u_camera: Camera;

[[group(2), binding(0)]]
var<uniform> u_body_details: BodyDetails;

[[group(3), binding(0)]]
var<uniform> u_light: Light;

// Vertex Shader
[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    let w = u_body_details.model;
    let model_space = u_body_details.model * vec4<f32>(in.position, 1.0); // world_pos
    
    // Output
    var out: VertexOutput;
    out.tex_coord = in.tex_coord;
    out.normal = mat3x3<f32>(w.x.xyz, w.y.xyz, w.z.xyz) * in.normal;
    out.vertex_position = model_space.xyz;
    out.position = u_camera.view_proj * model_space;
    out.raw_position = in.position;
    out.temp_k = u_body_details.temp_k;
    out.atmosphere_density = u_body_details.atmosphere_density;
    
    return out;
}

fn gamma_correct(color: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    return pow(color, vec3<f32>(1.0 / gamma));
}

fn star_main(in: VertexOutput, tex_scample: vec4<f32>) -> vec4<f32> {
    
    let sun_color = vec3<f32>(244.0, 128.0, 55.0) / 255.0;
    
    var n = (noise3_func(in.raw_position / 600.0, 4, 40.0, 0.7) + 1.0) * 0.5;
    let noise = vec3<f32>(n, n, n);
    
    let final_color = mix(sun_color, noise, vec3<f32>(0.01)) * 0.5;
    
    return vec4<f32>(gamma_correct(final_color), 1.0);
}

// Fragment Shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    
    // Determine the color of this pixel based on the texture coords
    let object_color = textureSample(u_diffuse_texture, u_sampler, in.tex_coord);
    
    if (in.temp_k >= 1000.0) {
        let star = star_main(in, object_color);
        return star;
    }
    
    // We don't need (or want) much ambient light, so 0.1 is fine
    let ambient_strength: f32 = 0.001;
    let ambient_color: vec3<f32> = u_light.color.xyz * ambient_strength;
    
    // Diffuse
    let normal = normalize(in.normal);
    let light_dir = normalize(u_light.position.xyz - in.vertex_position);

    let diffuse_strength: f32 = max(dot(normal, light_dir), 0.0);
    let diffuse_color: vec3<f32> = u_light.color.xyz * diffuse_strength;

    let result: vec3<f32> = (ambient_color + diffuse_color) * object_color.xyz;

    // Since lights don't typically (afaik) cast transparency, so we use
    // the alpha here at the end.
    return vec4<f32>(gamma_correct(result), object_color.a);
}
