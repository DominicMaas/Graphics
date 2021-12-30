use crate::Engine;

#[allow(unused_variables)]
pub trait GameObject {
    /// Render your UI to the main render pass, multiple render passes are not yet supported
    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, engine: &Engine) {}
}
