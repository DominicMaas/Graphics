// Vertex input and output
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] tex_coord: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] vertex_position: vec3<f32>;  
};

// Data structures 
[[block]]
struct Camera {
    view_proj: mat4x4<f32>;
};

[[block]]
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
    return out;
}

// ---------------------------------------------------------------- //

[[group(2), binding(0)]]
var u_diffuse_texture: texture_2d<f32>;

[[group(2), binding(1)]]
var u_sampler: sampler;

// Fragment Shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(u_diffuse_texture, u_sampler, in.tex_coord);
}