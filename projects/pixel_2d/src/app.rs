use crate::world::World;
use vesta::wgpu::RenderPass;
use vesta::winit::dpi::PhysicalSize;
use vesta::winit::event::{MouseButton, VirtualKeyCode};
use vesta::Engine;

pub struct App {
    pixel_pipeline: vesta::wgpu::RenderPipeline,
    camera: vesta::Camera,
    camera_controller: vesta::CameraController,
    world: World,
}

impl vesta::VestaApp for App {
    fn init(engine: &mut Engine) -> Self {
        // Create a layout with two uniform buffers (camera, and model)
        let render_pipeline_layout =
            engine
                .renderer
                .device
                .create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
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
                        &vesta::Texture::create_bind_group_layout(&engine.renderer.device),
                    ],
                    push_constant_ranges: &[],
                });

        // Pipeline / shader for pixels
        let pixel_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.swap_chain_desc.format,
            "Main Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("resources/pixel_shader.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        .with_cull_mode(None) // TODO: Fix rendering and remove this
        .build(&engine.renderer.device)
        .unwrap();

        // Camera which will let us see around the world
        let camera = vesta::Camera::new(
            (0.0, 0.0, 0.0).into(),
            vesta::OrthographicProjection::new(
                engine.get_window_size().width,
                engine.get_window_size().height,
                0.0001,
                1000.0,
            ),
            &engine.renderer.device,
        );

        let camera_controller = vesta::CameraController::new(5.0, 0.2);

        let world = World::new(&engine.renderer);

        Self {
            pixel_pipeline,
            camera,
            camera_controller,
            world,
        }
    }
    
    fn update(&mut self, engine: &mut vesta::Engine) {  
        self.camera_controller.process_input(&engine.io);    
        self.camera_controller.update_camera(&mut self.camera, &engine, false);
        
        self.camera.update_uniforms(&engine.renderer);
        
        if engine.io.mouse.get_button_down(MouseButton::Left) {
            let pos = engine.io.mouse.get_position();
            
            println!("Left mouse button was clicked at x:{}, y:{}", pos.x, pos.y);
        }
        
        if engine.io.keyboard.get_key_down(VirtualKeyCode::R) {
            println!("Rebuilding world...");
            self.world.rebuild(&engine.renderer);
        }
    }

    fn render<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, _engine: &Engine) {
        render_pass.set_pipeline(&self.pixel_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);

        self.world.draw(render_pass);
    }

    fn resize(&mut self, size: PhysicalSize<u32>, _engine: &vesta::Engine) {
        self.camera.projection.resize(size.width, size.height);
    }
}
