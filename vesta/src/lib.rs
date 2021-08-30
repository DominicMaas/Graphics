pub mod config;
pub mod engine;
pub mod math;
pub mod mesh;
pub mod projections;
pub mod render_pipeline;
pub mod renderer;
pub mod texture;
pub mod uniform_buffer;
pub mod vertex;

pub mod camera;
pub mod frustum;
pub mod io;

pub use camera::*;
pub use config::*;
pub use engine::*;
pub use frustum::*;
pub use math::*;
pub use mesh::*;
pub use projections::*;
pub use render_pipeline::*;
pub use renderer::*;
pub use texture::*;
pub use uniform_buffer::*;
pub use vertex::*;

// These are exposed in the vesta API, so allow dependents to access them
pub extern crate bytemuck;
pub extern crate cgmath;
pub extern crate image;
pub extern crate imgui;
pub extern crate wgpu;
pub extern crate winit;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
#[allow(unused_variables)]
pub trait VestaApp {
    /// Required: Create the application / game
    /// Engine is mutable here on purpose
    fn init(engine: &mut Engine) -> Self;

    /// Optional: Update events. This is called multiple times per frame
    /// Engine is mutable here on purpose
    fn physics_update(&mut self, dt: f32, engine: &mut Engine) {}

    /// Optional: Update events. This is called once per frame
    /// Engine is mutable here on purpose
    fn update(&mut self, engine: &mut Engine) {}

    /// Optional: Render your UI in this method using imgui
    fn render_ui(&mut self, ui: &imgui::Ui, engine: &Engine) {}

    /// Required: Render your UI to the main render pass, multiple
    /// render passes are not yet supported
    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, engine: &Engine);

    /// Optional: Called when the window is resized, update your camera matrixes here
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>, engine: &Engine) {}
}
