use crate::pixel::Pixel;
use crate::pixel::PixelType;
use crate::world::World;
use vesta::cgmath::Vector2;
use vesta::cgmath::Vector3;
use vesta::wgpu::RenderPass;
use vesta::winit::dpi::PhysicalSize;
use vesta::winit::event::{MouseButton, VirtualKeyCode};
use vesta::Engine;

pub struct App {
    pixel_pipeline: vesta::wgpu::RenderPipeline,
    camera: vesta::Camera,
    camera_controller: vesta::FpsCameraController,
    world: World,
    brush_size: i32,
    brush_type: PixelType,
    selected_pixel: Option<Pixel>,
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
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::Texture::create_bind_group_layout(&engine.renderer.device),
                    ],
                    push_constant_ranges: &[],
                });

        // Pipeline / shader for pixels
        let pixel_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
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
        let camera = vesta::CameraBuilder::new()
            .with_position((0.0, 0.0, 100.0).into())
            .build(
                vesta::OrthographicProjection::new(
                    engine.get_window_size().width,
                    engine.get_window_size().height,
                    0.0001,
                    1000.0,
                ),
                &engine.renderer.device,
            );

        let camera_controller = vesta::FpsCameraController::default();

        let world = World::new(&engine.renderer);

        Self {
            pixel_pipeline,
            camera,
            camera_controller,
            world,
            brush_size: 1,
            brush_type: PixelType::Water,
            selected_pixel: None,
        }
    }

    fn update(&mut self, engine: &mut vesta::Engine) {
        self.camera_controller.process_input(
            &mut self.camera,
            &engine,
            engine.is_cursor_captured(),
        );

        self.camera_controller.update_camera(&mut self.camera);
        self.camera.update_uniforms(&engine.renderer);

        if engine.io.mouse.get_button(MouseButton::Left) {
            let pos = engine.io.mouse.get_position_f32();
            let world_pos = self
                .camera
                .screen_to_world_point(Vector3::new(pos.x, pos.y, 0.0001));

            self.world.paint(
                self.brush_type,
                self.brush_size,
                Vector2::new(world_pos.x, world_pos.y),
            );
        }

        if engine.io.mouse.get_button_down(MouseButton::Right) {
            let pos = engine.io.mouse.get_position_f32();
            let world_pos = self
                .camera
                .screen_to_world_point(Vector3::new(pos.x, pos.y, 0.0001));

            self.selected_pixel = self.world.get_pixel(Vector2::new(world_pos.x, world_pos.y));
        }

        if engine.io.keyboard.get_key_down(VirtualKeyCode::R) {
            println!("Adding snow...");
            self.world.add_snow();
        }

        self.world.update(&engine);
        self.world.rebuild(&engine.renderer);
    }

    fn render_ui(&mut self, ctx: &vesta::egui::CtxRef, _engine: &Engine) {
        vesta::egui::Window::new("Toolbox")
            .show(&ctx, |ui| {
                ui.add(vesta::egui::Slider::new(&mut self.brush_size, 1..=100));

                ui.radio_value(&mut self.brush_type, PixelType::Air, "Air");
                ui.radio_value(&mut self.brush_type, PixelType::Snow, "Snow");
                ui.radio_value(&mut self.brush_type, PixelType::Water, "Water");
                ui.radio_value(&mut self.brush_type, PixelType::Sand, "Sand");
                ui.radio_value(&mut self.brush_type, PixelType::Ground, "Ground");

                match self.selected_pixel {
                    Some(pixel) => {
                        ui.separator();
                        ui.heading("Selected Pixel");
                        ui.label(format!("Type: {:?}", pixel.get_type()));
                        ui.label(format!(
                            "Color: {},{},{}",
                            pixel.get_color().r,
                            pixel.get_color().g,
                            pixel.get_color().b
                        ));
                        ui.label(format!(
                            "Velocity: {},{}:",
                            pixel.velocity.x,
                            pixel.velocity.y
                        ));
                    }
                    None => {}
                }
            });
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
