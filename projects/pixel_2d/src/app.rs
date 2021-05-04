use crate::world::World;
use vesta::wgpu::RenderPass;
use vesta::Engine;
use vesta::winit::dpi::PhysicalSize;
use vesta::winit::event::{DeviceEvent, VirtualKeyCode, WindowEvent, KeyboardInput};

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
        .with_shader_source(vesta::wgpu::util::make_spirv(include_bytes!(
            "resources/pixel_shader.spv"
        )))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        // Camera which will let us see around the world
        let camera = vesta::Camera::new(
            (0.0, 0.0, 1.0).into(),
            vesta::OrthographicProjection::new(
                engine.window_size.width,
                engine.window_size.height,
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

    fn update(&mut self, dt: f32, engine: &vesta::Engine) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera.update_uniforms(&engine.renderer);
    }

    fn render<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, _engine: &Engine) {
        render_pass.set_pipeline(&self.pixel_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);

        self.world.draw(render_pass);
    }

    fn resize(&mut self, size: PhysicalSize<u32>, _engine: &vesta::Engine) {
        self.camera.projection.resize(size.width, size.height);
    }

    fn input(&mut self, event: &WindowEvent, engine: &mut vesta::Engine) -> bool {
        if !self.camera_controller.process_keyboard(event) {
            match event {
                WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(keycode), .. }, ..
                } => {
                    match keycode {
                        VirtualKeyCode::R => {
                            self.world.rebuild(&engine.renderer);
                            true
                        }
                        _ => false,
                    }
                }
                _ => false
            }
        } else {
            false
        }
    }

    fn device_input(&mut self, event: &DeviceEvent, engine: &mut vesta::Engine) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                //self.camera_controller.process_mouse(delta.0, delta.1);
                true
            }
            _ => false,
        }
    }
}
