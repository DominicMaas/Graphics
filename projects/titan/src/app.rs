use vesta::{
    cgmath::{num_traits::FloatConst, SquareMatrix, Vector4},
    winit::{
        dpi::PhysicalSize,
        event::{MouseButton, VirtualKeyCode},
    },
};

use rand::Rng;

use crate::{cube::Cube, sky_shader::SkyShader, world::World};

pub struct App {
    chunk_render_pipeline: vesta::wgpu::RenderPipeline,
    chunk_render_pipeline_wire_frame: vesta::wgpu::RenderPipeline,
    general_render_pipeline: vesta::wgpu::RenderPipeline,
    sky_shader: SkyShader,
    camera: vesta::Camera,
    camera_controller: vesta::FpsCameraController,
    world: World,
    marker: Cube,
    is_wire_frame: bool,
}

impl vesta::VestaApp for App {
    fn init(engine: &mut vesta::Engine) -> Self {
        let general_pipeline_layout =
            engine
                .renderer
                .device
                .create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
                    label: Some("General Pipeline Layout"),
                    bind_group_layouts: &[
                        // Camera Uniform Buffer
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX | vesta::wgpu::ShaderStages::FRAGMENT,
                            &engine.renderer.device,
                        ),
                        // Chunk Uniform buffer
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                    ],
                    push_constant_ranges: &[],
                });

        // Layout for shaders
        let render_pipeline_layout =
            engine
                .renderer
                .device
                .create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        // Camera Uniform Buffer
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX | vesta::wgpu::ShaderStages::FRAGMENT,
                            &engine.renderer.device,
                        ),
                        // Chunk Uniform buffer
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        // Chunk Texture
                        &vesta::Texture::create_bind_group_layout(&engine.renderer.device),
                    ],
                    push_constant_ranges: &[],
                });

        // Render pipeline for shaders
        let chunk_render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "Chunk Render Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("res/chunk_shader.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        let chunk_render_pipeline_wire_frame = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "Chunk Render Pipeline (Debug)",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("res/chunk_shader.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        .with_topology(vesta::wgpu::PrimitiveTopology::LineList)
        .build(&engine.renderer.device)
        .unwrap();

        let general_render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "General Render Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("res/general_shader.wgsl").into(),
        ))
        .with_layout(&general_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        // Setup the main camera
        let camera = vesta::CameraBuilder::new()
            .with_position((0.0, 0.0, 0.0).into())
            .with_uniform_buffer_visibility(
                vesta::wgpu::ShaderStages::VERTEX | vesta::wgpu::ShaderStages::FRAGMENT,
            )
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

        let camera_controller = vesta::FpsCameraController::default();

        let mut rng = rand::thread_rng();
        let world = World::new(&engine.renderer, rng.gen());

        let sky_shader = SkyShader::new(&engine);

        // Init the engine
        Self {
            chunk_render_pipeline,
            chunk_render_pipeline_wire_frame,
            general_render_pipeline,
            sky_shader,
            camera,
            camera_controller,
            world,
            is_wire_frame: false,
            marker: Cube::new(&engine.renderer),
        }
    }

    fn update(&mut self, engine: &mut vesta::Engine) {
        // Update the world
        self.world.update(&engine.renderer, &self.camera);

        self.camera_controller.process_input(
            &mut self.camera,
            &engine,
            engine.is_cursor_captured(),
        );

        self.camera_controller.update_camera(&mut self.camera);
        self.camera.update_uniforms(&engine.renderer);

        self.sky_shader.uniform_buffer.data.view = self.camera.calc_matrix();
        self.sky_shader.uniform_buffer.data.cam_pos = Vector4::new(
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z,
            0.0,
        );

        self.sky_shader.update(&engine.renderer);

        // Add ability to escape out of camera
        if engine.io.keyboard.get_key_down(VirtualKeyCode::Escape) && engine.is_cursor_captured() {
            engine.set_cursor_captured(false);
        }

        // Add ability to capture camera again
        if engine.io.mouse.get_button_down(MouseButton::Left) && !engine.is_cursor_captured() {
            engine.set_cursor_captured(true);
        }

        self.marker.update(&self.camera, &engine.renderer);
    }

    fn render_ui(&mut self, ctx: &vesta::egui::CtxRef, _engine: &vesta::Engine) {
        let cam = &self.camera;
        let sky_shader = &mut self.sky_shader;
        let rendered_chunks = &self.world.rendered_chunks;
        let is_debug = &mut self.is_wire_frame;

        vesta::egui::Window::new("Toolbox")
            .show(&ctx, |ui| {
                ui.heading("Camera");
                ui.label(format!("Position: {:.2}, {:.2}, {:.2}", cam.position.x, cam.position.y, cam.position.z));
                ui.label(format!("Pitch: {:.2}", cam.pitch.0));
                ui.label(format!("Yaw: {:.2},", cam.yaw.0));

                ui.separator();

                ui.label(format!("Rendered Chunks: {}", rendered_chunks));

                ui.separator();

                ui.label("Sky Scatter Amount");
                ui.add(vesta::egui::Slider::new(&mut sky_shader.frag_uniform_buffer.data.scatter_amount, 0.0..=1.0));

                ui.separator();

                ui.add(vesta::egui::Checkbox::new(is_debug, "Show Wireframe"));
            });
    }

    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        engine: &vesta::Engine,
    ) {
        if self.is_wire_frame {
            render_pass.set_pipeline(&self.chunk_render_pipeline_wire_frame);
        } else {
            render_pass.set_pipeline(&self.chunk_render_pipeline);
        }

        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);
        self.world.render(render_pass, engine, &self.camera);

        self.sky_shader.render(render_pass, engine);

        render_pass.set_pipeline(&self.general_render_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);
        self.marker.render(render_pass);
    }

    fn resize(&mut self, size: PhysicalSize<u32>, engine: &vesta::Engine) {
        // The screen projection needs to be updated
        self.camera.projection.resize(size.width, size.height);

        self.sky_shader.uniform_buffer.data.proj = self.camera.projection.calc_matrix();
        self.sky_shader.uniform_buffer.data.proj_inv =
            self.camera.projection.calc_matrix().invert().unwrap();

        self.sky_shader.update(&engine.renderer);
    }
}
