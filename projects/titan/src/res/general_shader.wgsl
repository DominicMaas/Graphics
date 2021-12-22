// Vertex input and output
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] tex_coord: vec2<f32>;
    [[location(3)]] normal: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
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
    out.position = u_camera.view_proj * u_model.model * vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}

// ---------------------------------------------------------------- //

// Fragment Shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.7);    
}
