pub mod config;
pub mod engine;
pub mod renderer;
pub mod texture;
pub mod render_pipeline;
pub mod vertex;
pub mod mesh;
pub mod shader_loader;
pub mod uniform_buffer;
pub mod camera;

use cgmath::{Matrix4, Rad};
pub use config::*;
pub use engine::*;
pub use renderer::*;
pub use texture::*;
pub use render_pipeline::*;
pub use vertex::*;
pub use mesh::*;
pub use shader_loader::*;
pub use uniform_buffer::*;
pub use camera::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

/// Holds the current projection of the program, this needs to be updated
/// whenever the window size changes
pub struct Projection {
    pub aspect: f32,
    pub fov: Rad<f32>,
    pub z_near: f32,
    pub z_far: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fov: Rad<f32>, z_near: f32, z_far: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fov,
            z_near,
            z_far,
        }
    }

    /// When the window resizes, this updates the internal
    /// aspect ratio to ensure everything still looks correct
    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    /// Calculate the projection matrix for the window
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX
            * cgmath::perspective(self.fov, self.aspect, self.z_near, self.z_far)
    }
}

pub trait VestaApp {
    fn init(engine: &Engine) -> Self;
    fn update(&mut self, dt: std::time::Duration, engine: &Engine);
    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, engine: &Engine);
    
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>, engine: &Engine);
    
    fn input(&mut self, event: &winit::event::WindowEvent, engine: &Engine) -> bool;
    fn device_input(&mut self, event: &winit::event::DeviceEvent, engine: &Engine) -> bool;
}