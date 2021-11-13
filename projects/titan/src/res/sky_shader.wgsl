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

[[block]]
struct FragData {
    scatter_amount: f32;
};

[[group(0), binding(0)]]
var<uniform> r_data: Data;

[[group(1), binding(0)]]
var<uniform> frag_data: FragData;

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

fn get_sky(uv: vec2<f32>, scatter: f32) -> vec3<f32>
{
    let atmosphere = 1.0 - uv.y; //sqrt(1.0 - uv.y);
    let sky_color = vec3<f32>(0.2, 0.4, 0.8);
    
   // let scatter = pow(iMouse.y / iResolution.y,1.0 / 15.0);
   // scatter = 1.0 - clamp(scatter,0.8,1.0);
    
    let scatterColor = mix(vec3<f32>(1.0), vec3<f32>(1.0,0.3,0.0) * 1.5, vec3<f32>(scatter, scatter, scatter));
    
    let val = atmosphere / 2.3;
    return mix(sky_color, scatterColor, vec3<f32>(val, val, val));
}

//mat4 modelViewMatrix = modelUBO.model * sceneUBO.view;
[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    
    let sky = get_sky(in.uv.xy, frag_data.scatter_amount);
    
    return vec4<f32>(sky.x, sky.y, sky.z, 1.0);
}
