use vesta::{
    cgmath::{num_traits::FloatConst, Vector3},
    imgui::{self, im_str},
    winit::{
        dpi::PhysicalSize,
        event::{MouseButton, VirtualKeyCode},
    },
};

use crate::world::Chunk;

pub struct App {
    chunk_render_pipeline: vesta::wgpu::RenderPipeline,
    camera: vesta::Camera,
    camera_controller: vesta::CameraControllerTitan,
    chunk: Chunk,
    block_map_texture: vesta::Texture,
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
                            vesta::wgpu::ShaderStage::VERTEX,
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

        let camera_controller = vesta::CameraControllerTitan::new();

        let chunk = Chunk::new(Vector3::new(0.0, 0.0, 0.0), &engine.renderer); // Temp

        let block_map_texture = engine
            .renderer
            .create_texture_from_bytes(
                include_bytes!("res/img/block_map.png"),
                Some("res/img/block_map.png"),
            )
            .unwrap();

        // Init the engine
        Self {
            chunk_render_pipeline,
            camera,
            camera_controller,
            chunk,
            block_map_texture,
        }
    }

    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        engine: &vesta::Engine,
    ) {
        render_pass.set_pipeline(&self.chunk_render_pipeline);
        render_pass.set_bind_group(0, &self.camera.uniform_buffer.bind_group, &[]);
        render_pass.set_bind_group(2, &self.block_map_texture.bind_group.as_ref().unwrap(), &[]);

        self.chunk.render(render_pass, engine);
    }

    fn update(&mut self, engine: &mut vesta::Engine) {
        // Process Chunk
        match self.chunk.get_state() {
            crate::world::ChunkState::Created => self.chunk.load(),
            crate::world::ChunkState::Dirty => self.chunk.rebuild(&engine.renderer),
            _ => {}
        }

        self.camera_controller.process_input(
            &mut self.camera,
            &engine,
            engine.is_cursor_captured(),
        );
        self.camera_controller.update_camera(&mut self.camera);

        self.camera.update_uniforms(&engine.renderer);

        // Add ability to escape out of camera
        if engine.io.keyboard.get_key_down(VirtualKeyCode::Escape) && engine.is_cursor_captured() {
            engine.set_cursor_captured(false);
        }

        // Add ability to capture camera again
        if engine.io.mouse.get_button_down(MouseButton::Left) && !engine.is_cursor_captured() {
            engine.set_cursor_captured(true);
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>, _engine: &vesta::Engine) {
        // The screen projection needs to be updated
        self.camera.projection.resize(size.width, size.height);
    }

    fn render_ui(&mut self, ui: &imgui::Ui, _engine: &vesta::Engine) {
        let cam = &self.camera;
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

                cg.end(&ui);
            });
    }
}
