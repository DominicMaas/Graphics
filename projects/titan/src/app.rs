use vesta::{
    cgmath::{num_traits::FloatConst, SquareMatrix, Vector4},
    imgui::{self, im_str},
    winit::{
        dpi::PhysicalSize,
        event::{MouseButton, VirtualKeyCode},
    },
};

use rand::Rng;

use crate::{sky_shader::SkyShader, world::World};

pub struct App {
    chunk_render_pipeline: vesta::wgpu::RenderPipeline,
    sky_shader: SkyShader,
    camera: vesta::Camera,
    camera_controller: vesta::CameraControllerTitan,
    world: World,
}

impl vesta::VestaApp for App {
    fn init(engine: &mut vesta::Engine) -> Self {
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
                            vesta::wgpu::ShaderStage::VERTEX | vesta::wgpu::ShaderStage::FRAGMENT,
                            &engine.renderer.device,
                        ),
                        // Chunk Uniform buffer
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStage::VERTEX,
                            &engine.renderer.device,
                        ),
                        // Chunk Texture
                        &vesta::Texture::create_bind_group_layout(&engine.renderer.device),
                    ],
                    push_constant_ranges: &[],
                });

        // Render pipeline for shaders
        let chunk_render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.swap_chain_desc.format,
            "Chunk Render Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("res/chunk_shader.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        // Setup the main camera
        let camera = vesta::CameraBuilder::new()
            .with_position((0.0, 0.0, 0.0).into())
            .with_uniform_buffer_visibility(
                vesta::wgpu::ShaderStage::VERTEX | vesta::wgpu::ShaderStage::FRAGMENT,
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

        let camera_controller = vesta::CameraControllerTitan::new();

        let mut rng = rand::thread_rng();
        let seed = rng.gen();

        let world = World::new(&engine.renderer, seed);

        let sky_shader = SkyShader::new(&engine);

        // Init the engine
        Self {
            chunk_render_pipeline,
            sky_shader,
            camera,
            camera_controller,
            world,
        }
    }

    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        engine: &vesta::Engine,
    ) {
        render_pass.set_pipeline(&self.chunk_render_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);

        self.world.render(render_pass, engine, &self.camera);
        self.sky_shader.render(render_pass, engine);
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
    }

    fn resize(&mut self, size: PhysicalSize<u32>, engine: &vesta::Engine) {
        // The screen projection needs to be updated
        self.camera.projection.resize(size.width, size.height);

        self.sky_shader.uniform_buffer.data.proj = self.camera.projection.calc_matrix();
        self.sky_shader.uniform_buffer.data.proj_inv =
            self.camera.projection.calc_matrix().invert().unwrap();

        self.sky_shader.update(&engine.renderer);
    }

    fn render_ui(&mut self, ui: &imgui::Ui, _engine: &vesta::Engine) {
        let cam = &self.camera;
        let sky_shader = &mut self.sky_shader;
        let rendered_chunks = &self.world.rendered_chunks;

        let window = vesta::imgui::Window::new(im_str!("Toolbox"));
        window
            .size([300.0, 300.0], vesta::imgui::Condition::FirstUseEver)
            .build(&ui, || {
                let cg = ui.begin_group();
                ui.text(vesta::imgui::im_str!("Camera:"));
                ui.text(vesta::imgui::im_str!(
                    "Position: {:.2}, {:.2}, {:.2}",
                    cam.position.x,
                    cam.position.y,
                    cam.position.z
                ));
                ui.text(vesta::imgui::im_str!("Pitch: {:.2} rad", cam.pitch.0));
                ui.text(vesta::imgui::im_str!("Yaw: {:.2} rad", cam.yaw.0));

                ui.separator();

                ui.text(vesta::imgui::im_str!(
                    "Rendered Chunks: {}",
                    rendered_chunks
                ));

                ui.separator();

                imgui::Slider::new(im_str!("Sky Scatter Amount"))
                    .range(0.0..=1.0)
                    .build(&ui, &mut sky_shader.frag_uniform_buffer.data.scatter_amount);

                cg.end(&ui);
            });
    }
}
