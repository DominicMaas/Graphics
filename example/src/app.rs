use std::borrow::Cow;
use cgmath::Vector3;
use vesta::DrawMesh;
pub struct App {
    render_pipeline: wgpu::RenderPipeline,
    mesh: vesta::Mesh
}

impl vesta::VestaApp for App {
    fn init(engine: &vesta::Engine) -> Self {
        let render_pipeline_layout =
            engine.renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
                    
        let render_pipeline = vesta::RenderPipelineBuilder::new(engine.renderer.swap_chain_desc.format, "Render Pipeline")
            .with_vertext_shader_source(wgpu::ShaderSource::SpirV(Cow::Borrowed(vesta::ShaderLoader::load_wgsl(include_str!("shader.wgsl")).unwrap().as_slice())))
            .with_fragment_shader_source(wgpu::ShaderSource::SpirV(Cow::Borrowed(vesta::ShaderLoader::load_wgsl(include_str!("shader.wgsl")).unwrap().as_slice())))
            .with_layout(&render_pipeline_layout)
            .with_topology(wgpu::PrimitiveTopology::PointList)
            .build(&engine.renderer.device)
            .unwrap();
                                    
        let mut vertices: Vec<vesta::Vertex> = Vec::new();
        vertices.push(vesta::Vertex::with_color(Vector3::new(0.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0)));
        vertices.push(vesta::Vertex::with_color(Vector3::new(-0.5, -0.5, 0.0), Vector3::new(0.0, 1.0, 0.0)));
        vertices.push(vesta::Vertex::with_color(Vector3::new(0.5, -0.5, 0.0), Vector3::new(0.0, 0.0, 1.0)));
          
        let mesh = engine.renderer.create_mesh(vertices, Vec::new());
            
        Self {
            render_pipeline,
            mesh
        }            
    }
    
    fn update(&mut self, _engine: &vesta::Engine) {
        
    }
    
    fn render<'a>(&'a mut self, _engine: &vesta::Engine, render_pass: &mut wgpu::RenderPass<'a>) {        
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw_mesh(&self.mesh);
    }
}
