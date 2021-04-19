use vesta::wgpu::RenderPass;
use vesta::Engine;

pub struct App {}

impl vesta::VestaApp for App {
    fn init(engine: &mut Engine) -> Self {
        Self { }
    }

    fn render<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, engine: &Engine) {
    }
}
