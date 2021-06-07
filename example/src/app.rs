use vesta::{winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, WindowEvent},
}};
use vesta::{
    cgmath::num_traits::FloatConst,
    winit::event::{KeyboardInput, VirtualKeyCode},
};

use crate::cube::Cube;

pub struct App {
    render_pipeline: vesta::wgpu::RenderPipeline,
    cube: Cube,
    camera: vesta::Camera,
    camera_controller: vesta::CameraController,
}

impl vesta::VestaApp for App {
    fn init(engine: &mut vesta::Engine) -> Self {
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

        let render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.swap_chain_desc.format,
            "Render Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("shader.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        let cube = Cube::new(&engine.renderer);

        // Setup the main camera
        let camera = vesta::Camera::new(
            (0.0, 0.0, 0.0).into(),
            vesta::PerspectiveProjection::new(
                engine.get_window_size().width,
                engine.get_window_size().height,
                vesta::cgmath::Rad(70.0 / 180.0 * f32::PI()),
                0.01,
                1000.0,
            ),
            &engine.renderer.device,
        );

        let camera_controller = vesta::CameraController::new(32.0, 0.2);

        engine.set_cursor_captured(true);

        Self {
            render_pipeline,
            cube,
            camera,
            camera_controller,
        }
    }
    
    fn physics_update(&mut self, dt: f32, engine: &mut vesta::Engine) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.cube.update(dt, &engine.renderer);
    }

    fn update(&mut self, engine: &mut vesta::Engine) {    
        self.camera.update_uniforms(&engine.renderer);
   
        let mouse_pos = engine.io.mouse.get_position();
        println!("x: {}, y: {}", mouse_pos.x, mouse_pos.y);
    }

    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        _engine: &vesta::Engine,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);
        self.cube.draw(render_pass);
    }

    fn resize(&mut self, size: PhysicalSize<u32>, _engine: &vesta::Engine) {
        // The screen projection needs to be updated
        self.camera.projection.resize(size.width, size.height);
    }

    fn input(&mut self, event: &WindowEvent, engine: &mut vesta::Engine) -> bool {
        if !self.camera_controller.process_keyboard(event) {
            match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => match keycode {
                    VirtualKeyCode::Escape => {
                        engine.set_cursor_captured(false);
                        true
                    }
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        }
    }

    fn device_input(&mut self, event: &DeviceEvent, engine: &mut vesta::Engine) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if engine.is_cursor_captured() {
                    self.camera_controller.process_mouse(delta.0, delta.1);
                }

                true
            }
            _ => false,
        }
    }
}
