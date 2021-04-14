// Vertex input and output
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] tex_coord: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
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

[[group(0), binding(0)]]
var u_camera: Camera;

[[group(1), binding(0)]]
var u_model: Model;

// Vertex Shader
[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    
    var out: VertexOutput;
    out.position = u_camera.view_proj * vec4<f32>(in.position, 1.0);
    return out;
}

// Fragment Shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    //return vec4<f32>(in.color, 1.0);
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}