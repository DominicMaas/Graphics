use std::borrow::Cow;
use vesta::cgmath::num_traits::FloatConst;
use vesta::winit::{dpi::PhysicalSize, event::{DeviceEvent, WindowEvent}};

use crate::cube::Cube;

pub struct App {
    render_pipeline: vesta::wgpu::RenderPipeline,
    cube: Cube,
    camera: vesta::Camera,
    camera_controller: vesta::CameraController
}

impl vesta::VestaApp for App {
    fn init(engine: &vesta::Engine) -> Self {
        let render_pipeline_layout =
            engine.renderer.device.create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &vesta::UniformBufferUtils::create_bind_group_layout(
                        vesta::wgpu::ShaderStage::VERTEX,
                        &engine.renderer.device,
                    ),
                    &vesta::UniformBufferUtils::create_bind_group_layout(
                        vesta::wgpu::ShaderStage::VERTEX,
                        &engine.renderer.device,
                    ),
                    &vesta::Texture::create_bind_group_layout(&engine.renderer.device)
                ],
                push_constant_ranges: &[],
            });
                    
        let render_pipeline = vesta::RenderPipelineBuilder::new(engine.renderer.swap_chain_desc.format, "Render Pipeline")
            .with_shader_source(vesta::wgpu::ShaderSource::SpirV(Cow::Borrowed(vesta::ShaderLoader::load_wgsl(include_str!("shader.wgsl")).unwrap().as_slice())))
            .with_layout(&render_pipeline_layout)
            .build(&engine.renderer.device)
            .unwrap();
                                       
        let cube = Cube::new(&engine.renderer);
                
        // Setup the main camera
        let camera = vesta::Camera::new(
            (0.0, 0.0, 0.0).into(),
            vesta::PerspectiveProjection::new(
                engine.window_size.width,
                engine.window_size.height,
                vesta::cgmath::Rad(70.0 / 180.0 * f32::PI()),
                0.01,
                1000.0,
            ),
            &engine.renderer.device,
        );

        let camera_controller = vesta::CameraController::new(32.0, 0.2);
        
        Self {
            render_pipeline,
            cube,
            camera,
            camera_controller
        }            
    }
    
    fn update(&mut self, dt: f32, engine: &vesta::Engine) {
         // Update camera positions
         self.camera_controller.update_camera(&mut self.camera, dt);
         self.camera.update_uniforms(&engine.renderer);
         
         self.cube.update(dt, &engine.renderer);
    }
    
    fn render<'a>(&'a mut self, render_pass: &mut vesta::wgpu::RenderPass<'a>, _engine: &vesta::Engine) {        
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);
        self.cube.draw(render_pass);        
    }
    
    fn resize(&mut self, size: PhysicalSize<u32>, _engine: &vesta::Engine) {
         // The screen projection needs to be updated
         self.camera.projection.resize(size.width, size.height);
    }
    
    fn input(&mut self, event: &WindowEvent, _engine: &vesta::Engine) -> bool {
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
