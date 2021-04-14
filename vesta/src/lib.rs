pub mod config;
pub mod engine;
pub mod renderer;
pub mod texture;
pub mod render_pipeline;
pub mod vertex;
pub mod mesh;
pub mod shader_loader;

pub use config::*;
pub use engine::*;
pub use renderer::*;
pub use texture::*;
pub use render_pipeline::*;
pub use vertex::*;
pub use mesh::*;
pub use shader_loader::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub trait VestaApp {
    fn init(engine: &Engine) -> Self;
    fn update(&mut self, engine: &Engine);
    fn render<'a>(&'a mut self, engine: &Engine, render_pass: &mut wgpu::RenderPass<'a>);
}