// Vertex input and output
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] tex_coord: vec2<f32>;
};

struct VertexOutput {
    [[location(0)]] color: vec3<f32>;
    [[location(1)]] tex_coord: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

// Vertex Shader
[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = in.tex_coord;
    out.color = in.color;
    out.position = vec4<f32>(in.position, 1.0);
    return out;
}

// Fragment Shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}