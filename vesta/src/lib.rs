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
pub mod projections;

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
pub use projections::*;

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
    fn init(engine: &Engine) -> Self;
    
    /// Optional: Update events
    fn update(&mut self, dt: f32, engine: &Engine) { }
    
    /// Optional: Render your UI in this method using imgui
    fn render_ui(&mut self, ui: &imgui::Ui, engine: &Engine) { }
    
    /// Required: Render your UI to the main render pass, multiple 
    /// render passes are not yet supported
    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, engine: &Engine);
    
    /// Optional: Called when the window is resized, update your camera matrixes here
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>, engine: &Engine) { }
    
    /// Optional: Input Handling
    fn input(&mut self, event: &winit::event::WindowEvent, engine: &Engine) -> bool { false }
    
    /// Optional: Input handling
    fn device_input(&mut self, event: &winit::event::DeviceEvent, engine: &Engine) -> bool { false }
}