use vesta::renderer::Renderer;
use vesta::VestaApp;

pub struct App {}

impl App {
    pub fn new() -> Self { App {} }
}

impl VestaApp for App {
    fn init(&self, renderer: &Renderer) {
        
    }
    
    fn update(&self, renderer: &Renderer) {
        
    }
    
    fn render(&self, renderer: &Renderer, render_pass: &mut wgpu::RenderPass) {
        
    }
}