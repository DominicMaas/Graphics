// Vertex input and output
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] tex_coord: vec2<f32>;
    [[location(3)]] normal: vec3<f32>;
};

struct VertexOutput {
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] vertex_position: vec3<f32>;
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

[[block]]
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
var u_camera: Camera;

[[group(2), binding(0)]]
var u_model: Model;

[[group(3), binding(0)]]
var u_light: Light;

// Vertex Shader
[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    const w = u_model.model;
    const model_space = u_model.model * vec4<f32>(in.position, 1.0); // world_pos
    
    // Output
    var out: VertexOutput;
    out.tex_coord = in.tex_coord;
    out.normal = mat3x3<f32>(w.x.xyz, w.y.xyz, w.z.xyz) * in.normal;
    out.vertex_position = model_space.xyz;
    out.position = u_camera.view_proj * model_space;
    return out;
}

// Fragment Shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // Determine the color of this pixel based on the texture coords
    const object_color = textureSample(u_diffuse_texture, u_sampler, in.tex_coord);
    
    // We don't need (or want) much ambient light, so 0.1 is fine
    const ambient_strength: f32 = 0.1;
    const ambient_color: vec3<f32> = u_light.color.xyz * ambient_strength;
    
    // Diffuse
    const normal = normalize(in.normal);
    const light_dir = normalize(u_light.position.xyz - in.vertex_position);

    const diffuse_strength: f32 = max(dot(normal, light_dir), 0.0);
    const diffuse_color: vec3<f32> = u_light.color.xyz * diffuse_strength;

    const result: vec3<f32> = (ambient_color + diffuse_color) * object_color.xyz;

    // Since lights don't typically (afaik) cast transparency, so we use
    // the alpha here at the end.
    return vec4<f32>(result, object_color.a);
}