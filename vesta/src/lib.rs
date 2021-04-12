pub mod config;
pub mod engine;
pub mod renderer;
pub mod texture;

pub trait VestaApp {
    fn init(&self, renderer: &renderer::Renderer);
    fn update(&self, renderer: &renderer::Renderer);
    fn render(&self, renderer: &renderer::Renderer, render_pass: &mut wgpu::RenderPass);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
