struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] uv: vec3<f32>;
};

[[block]]
struct Data {
    // from camera to screen
    proj: mat4x4<f32>;
    // from screen to camera
    proj_inv: mat4x4<f32>;
    // from world to camera
    view: mat4x4<f32>;
    // camera position
    cam_pos: vec4<f32>;
};

[[group(0), binding(0)]]
var r_data: Data;

[[stage(vertex)]]
fn main([[builtin(vertex_index)]] vertex_index: u32) -> VertexOutput {
    // hacky way to draw a large triangle
    let tmp1 = i32(vertex_index) / 2;
    let tmp2 = i32(vertex_index) & 1;
    let pos = vec4<f32>(
        f32(tmp1) * 4.0 - 1.0,
        f32(tmp2) * 4.0 - 1.0,
        1.0,
        1.0
    );

    // transposition = inversion for this orthonormal matrix
    let inv_model_view = transpose(mat3x3<f32>(r_data.view.x.xyz, r_data.view.y.xyz, r_data.view.z.xyz));
    let unprojected = r_data.proj_inv * pos;

    var out: VertexOutput;
    out.uv = inv_model_view * unprojected.xyz;
    out.position = pos;
        
    return out;
}

//mat4 modelViewMatrix = modelUBO.model * sceneUBO.view;
 //   vec3 position = mat3(modelViewMatrix) * inPosition.xyz;
  //  gl_Position = (sceneUBO.proj * vec4( position, 0.0 )).xyzz;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}

