use vesta::{
    cgmath::num_traits::FloatConst, components::Light, components::Transform, log::info,
    winit::dpi::PhysicalSize, Engine, Entity,
};

use crate::cube::Cube;
use vesta::egui::CtxRef;

pub struct App {
    render_pipeline: vesta::wgpu::RenderPipeline,
    cube: Cube,
    camera: vesta::Camera,
    camera_controller: vesta::ArcBallCameraController,
}

impl vesta::VestaApp for App {
    fn init(engine: &mut vesta::Engine) -> Self {
        info!("Init Start!");

        let render_pipeline_layout =
            engine
                .renderer
                .device
                .create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &vesta::Texture::create_bind_group_layout(&engine.renderer.device),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "Render Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("shader.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        // Test???
        let mut light_entity = Entity::new(Transform {
            ..Default::default()
        });

        light_entity.add_component(Light {
            ..Default::default()
        });

        let cube = Cube::new(&engine.renderer);

        // Setup the main camera
        let camera = vesta::CameraBuilder::new()
            .with_position((0.0, 0.0, 3.0).into())
            .build(
                vesta::PerspectiveProjection::new(
                    engine.get_window_size().width,
                    engine.get_window_size().height,
                    vesta::cgmath::Rad(70.0 / 180.0 * f32::PI()),
                    0.01,
                    1000.0,
                ),
                &engine.renderer.device,
            );

        let camera_controller = vesta::ArcBallCameraController::default();

        info!("Init Finish!");

        Self {
            render_pipeline,
            cube,
            camera,
            camera_controller,
        }
    }

    fn physics_update(&mut self, dt: f32, engine: &mut vesta::Engine) {
        self.cube.update(dt, &engine.renderer);
    }

    fn update(&mut self, engine: &mut vesta::Engine) {
        self.camera_controller.process_input(
            &mut self.camera,
            &engine,
            engine.is_cursor_captured(),
        );

        self.camera_controller.update_camera(&mut self.camera);
        self.camera.update_uniforms(&engine.renderer);
    }

    fn render_ui(&mut self, ctx: &CtxRef, _engine: &Engine) {
        vesta::egui::SidePanel::left("settings_panel")
            .frame(vesta::egui::Frame {
                margin: vesta::egui::vec2(20.0, 20.0),
                ..Default::default()
            })
            .min_width(200.0)
            .show(&ctx, |ui| {
                ui.heading("Vesta Engine Example");
                ui.separator();
                ui.label("Click and drag to move the camera around the cube.");
            });
    }

    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        _engine: &vesta::Engine,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, &self.camera.uniform_buffer.bind_group, &[]);
        self.cube.draw(render_pass);
    }

    fn resize(&mut self, size: PhysicalSize<u32>, _engine: &vesta::Engine) {
        // The screen projection needs to be updated
        self.camera.projection.resize(size.width, size.height);
    }
}
