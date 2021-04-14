use std::borrow::Cow;
use cgmath::Vector3;
use vesta::DrawMesh;
use cgmath::num_traits::FloatConst;
use winit::event::DeviceEvent;

pub struct App {
    render_pipeline: wgpu::RenderPipeline,
    mesh: vesta::Mesh,
    camera: vesta::Camera,
    camera_controller: vesta::CameraController
}

impl vesta::VestaApp for App {
    fn init(engine: &vesta::Engine) -> Self {
        let render_pipeline_layout =
            engine.renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &vesta::UniformBufferUtils::create_bind_group_layout(
                        wgpu::ShaderStage::VERTEX,
                        &engine.renderer.device,
                    ),
                ],
                push_constant_ranges: &[],
            });
                    
        let render_pipeline = vesta::RenderPipelineBuilder::new(engine.renderer.swap_chain_desc.format, "Render Pipeline")
            .with_shader_source(wgpu::ShaderSource::SpirV(Cow::Borrowed(vesta::ShaderLoader::load_wgsl(include_str!("shader.wgsl")).unwrap().as_slice())))
            .with_layout(&render_pipeline_layout)
            .build(&engine.renderer.device)
            .unwrap();
                                    
        let mut vertices: Vec<vesta::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
          
        let x = -1.0;
        let y = -1.0;
        let z = -5.0;
        let curr_index = 0;
        
        vertices.push(vesta::Vertex::with_color(
            Vector3::new(1.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
        ));
        vertices.push(vesta::Vertex::with_color(
            Vector3::new(1.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
        ));
        vertices.push(vesta::Vertex::with_color(
            Vector3::new(0.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
        ));
        vertices.push(vesta::Vertex::with_color(
            Vector3::new(0.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
        ));

        indices.push(curr_index + 0);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);
        
        let mesh = engine.renderer.create_mesh(vertices, indices);
            
        // Setup the main camera
        let camera = vesta::Camera::new(
            (0.0, 0.0, 0.0).into(),
            vesta::Projection::new(
                engine.renderer.swap_chain_desc.width,
                engine.renderer.swap_chain_desc.height,
                cgmath::Rad(70.0 / 180.0 * f32::PI()),
                0.01,
                1000.0,
            ),
            &engine.renderer.device,
        );

        let camera_controller = vesta::CameraController::new(32.0, 0.2);
        
        Self {
            render_pipeline,
            mesh,
            camera,
            camera_controller
        }            
    }
    
    fn update(&mut self, dt: std::time::Duration, engine: &vesta::Engine) {
         // Update camera positions
         self.camera_controller.update_camera(&mut self.camera, dt);
         self.camera.update_uniforms(&engine.renderer.queue);
    }
    
    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, _engine: &vesta::Engine) {        
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);        
        render_pass.draw_mesh(&self.mesh);
    }
    
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>, _engine: &vesta::Engine) {
         // The screen projection needs to be updated
         self.camera.projection.resize(size.width, size.height);
    }
    
    fn input(&mut self, event: &winit::event::WindowEvent, _engine: &vesta::Engine) -> bool {
        self.camera_controller.process_keyboard(event)
    }
    
    fn device_input(&mut self, event: &DeviceEvent, _engine: &vesta::Engine) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.camera_controller.process_mouse(delta.0, delta.1);
                true
            }
            _ => false,
        }
    }
}
