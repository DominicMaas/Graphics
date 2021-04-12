use std::borrow::Cow;

pub struct App {
    render_pipeline: wgpu::RenderPipeline
}

impl vesta::VestaApp for App {
    fn init(renderer: &vesta::Renderer) -> Self {
        let render_pipeline_layout =
            renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        
        let render_pipeline = vesta::RenderPipelineBuilder::new(renderer.swap_chain_desc.format, "Render Pipeline")
            .with_shader_source(wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))))
            .with_layout(&render_pipeline_layout)
            .build(&renderer.device)
            .unwrap();
            
        Self {
            render_pipeline
        }            
    }
    
    fn update(&mut self, _renderer: &vesta::Renderer) {
        
    }
    
    fn render<'a>(&'a mut self, _renderer: &vesta::Renderer, render_pass: &mut wgpu::RenderPass<'a>) {        
        render_pass.set_pipeline(&self.render_pipeline);
    }
}