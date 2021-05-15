use vesta::cgmath::{Vector3, Vector4};

/// This custom universe uses this G
pub const G: f32 = 1.0e-7;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LightUniform {
    pub position: Vector4<f32>,
    pub color: Vector4<f32>,
}

unsafe impl vesta::bytemuck::Zeroable for LightUniform {}
unsafe impl vesta::bytemuck::Pod for LightUniform {}

impl LightUniform {
    pub fn new(position: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self {
            position: Vector4::new(position.x, position.y, position.z, 1.0),
            color: Vector4::new(color.x, color.y, color.z, 1.0),
        }
    }
}
