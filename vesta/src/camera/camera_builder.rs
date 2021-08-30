use crate::{Camera, Projection};
use cgmath::Vector3;

pub struct CameraBuilder<'a> {
    position: Vector3<f32>,
    uniform_buffer_name: &'a str,
    uniform_buffer_visibility: wgpu::ShaderStages,
}

impl<'a> CameraBuilder<'a> {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            uniform_buffer_name: "None",
            uniform_buffer_visibility: wgpu::ShaderStages::VERTEX,
        }
    }

    /// The initial position to place the camera
    pub fn with_position(&mut self, position: Vector3<f32>) -> &mut Self {
        self.position = position;
        self
    }

    /// Set a custom name for the uniform buffer
    pub fn with_uniform_buffer_name(&mut self, name: &'a str) -> &mut Self {
        self.uniform_buffer_name = name;
        self
    }

    /// Set a custom visibility for the uniform buffer
    pub fn with_uniform_buffer_visibility(&mut self, visibility: wgpu::ShaderStages) -> &mut Self {
        self.uniform_buffer_visibility = visibility;
        self
    }

    /// Build a camera with a projection and required device
    pub fn build(
        &mut self,
        projection: impl Projection + 'static,
        device: &wgpu::Device,
    ) -> Camera {
        Camera::new_internal(
            self.position,
            projection,
            self.uniform_buffer_visibility,
            self.uniform_buffer_name,
            device,
        )
    }
}

impl<'a> Default for CameraBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
