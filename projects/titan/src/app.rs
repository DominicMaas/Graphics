pub struct App {}

impl vesta::VestaApp for App {
    fn init(_engine: &mut vesta::Engine) -> Self {
        // Init the engine
        Self {}
    }

    fn render<'a>(
        &'a mut self,
        _render_pass: &mut vesta::wgpu::RenderPass<'a>,
        _engine: &vesta::Engine,
    ) {
        // Perform rendering
    }
}
