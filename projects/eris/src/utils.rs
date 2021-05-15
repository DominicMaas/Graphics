/// This custom universe uses this G
pub const G: f32 = 1.0e-7;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LightUniform {
    pub position: cgmath::Vector4<f32>,
    pub color: cgmath::Vector4<f32>,
}

unsafe impl bytemuck::Zeroable for LightUniform {}
unsafe impl bytemuck::Pod for LightUniform {}

impl LightUniform {
    pub fn new(position: cgmath::Vector3<f32>, color: cgmath::Vector3<f32>) -> Self {
        Self {
            position: cgmath::Vector4::new(position.x, position.y, position.z, 1.0),
            color: cgmath::Vector4::new(color.x, color.y, color.z, 1.0),
        }
    }
}