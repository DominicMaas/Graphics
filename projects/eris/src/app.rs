use std::time::Duration;

use crate::{c_body::CBody, utils::LightUniform};
use vesta::{
    cgmath::{num_traits::FloatConst, InnerSpace, Vector3},
    winit::event::{MouseButton, VirtualKeyCode},
    DrawMesh,
};

pub struct App {
    render_pipeline: vesta::wgpu::RenderPipeline,
    c_body_pipeline: vesta::wgpu::RenderPipeline,
    camera: vesta::Camera,
    camera_controller: vesta::FpsCameraController,
    bodies: Vec<CBody>,
    lights_uniform: vesta::UniformBuffer<LightUniform>,
}

impl vesta::VestaApp for App {
    fn init(engine: &mut vesta::Engine) -> Self {
        // Pipeline layout
        let render_pipeline_layout =
            engine
                .renderer
                .device
                .create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &vesta::Texture::create_bind_group_layout(&engine.renderer.device),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStage::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStage::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStage::VERTEX | vesta::wgpu::ShaderStage::FRAGMENT,
                            &engine.renderer.device,
                        ),
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.swap_chain_desc.format,
            "Main Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("shaders/main.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        let c_body_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.swap_chain_desc.format,
            "C Body Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("shaders/c_body.wgsl").into(),
        ))
        .with_layout(&render_pipeline_layout)
        //.with_topology(wgpu::PrimitiveTopology::LineList)
        .build(&engine.renderer.device)
        .unwrap();

        // Setup the main camera
        let camera = vesta::CameraBuilder::new()
            .with_position((0.0, 0.0, 0.0).into())
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

        let lights_uniform = vesta::UniformBuffer::new(
            "Light Uniform Buffer",
            vesta::wgpu::ShaderStage::VERTEX | vesta::wgpu::ShaderStage::FRAGMENT,
            LightUniform::new((2.0, 2.0, 2.0).into(), (1.0, 1.0, 1.0).into()),
            &engine.renderer.device,
        );

        // Bodies Setup
        let sun_texture = engine
            .renderer
            .create_texture_from_bytes(
                include_bytes!("images/sun.png"),
                Some("sun.png"),
                Default::default(),
            )
            .unwrap();
        let earth_texture = engine
            .renderer
            .create_texture_from_bytes(
                include_bytes!("images/earth.png"),
                Some("earth.png"),
                Default::default(),
            )
            .unwrap();
        let moon_texture = engine
            .renderer
            .create_texture_from_bytes(
                include_bytes!("images/earth.png"),
                Some("earth.png"),
                Default::default(),
            )
            .unwrap();

        let mut bodies = Vec::new();

        let sun = CBody::new(
            "Sun".to_string(),
            1000000.0,
            32.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            sun_texture,
            &engine.renderer.device,
        );

        let earth = CBody::new(
            "Earth".to_string(),
            10000.0,
            12.0,
            Vector3::new(200.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -sun.calculate_velocity_at_radius(200.0)),
            earth_texture,
            &engine.renderer.device,
        );

        let moon = CBody::new(
            "Moon".to_string(),
            0.1,
            2.0,
            Vector3::new(200.0 + 12.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -earth.calculate_velocity_at_radius(12.0)),
            moon_texture,
            &engine.renderer.device,
        );

        bodies.push(sun);
        bodies.push(earth);
        bodies.push(moon);

        engine.set_cursor_captured(true);

        Self {
            render_pipeline,
            c_body_pipeline,
            camera,
            camera_controller,
            bodies,
            lights_uniform,
        }
    }

    fn physics_update(&mut self, dt: f32, engine: &mut vesta::Engine) {
        // Loop through all bodies and apply updates
        for i in 0..self.bodies.len() {
            let (before, nonbefore) = self.bodies.split_at_mut(i);
            let (body, after) = nonbefore.split_first_mut().unwrap();

            // Calculate net force against other bodies

            // This loop iterates over all bodies that are no the current body
            for body2 in before.iter().chain(after.iter()) {
                let sqr_distance: f32 = (body2.position - body.position).magnitude2();
                let force_direction: Vector3<f32> = (body2.position - body.position).normalize();
                let force: Vector3<f32> =
                    force_direction * body.standard_gravitational_parameter() * body2.mass
                        / sqr_distance;
                let acceleration: Vector3<f32> = force / body.mass;

                body.velocity += acceleration;
            }

            // Run simulations
            body.update(Duration::from_secs_f32(dt));

            engine.renderer.queue.write_buffer(
                &body.uniform_buffer.buffer,
                0,
                vesta::bytemuck::cast_slice(&[body.uniform_buffer.data]),
            );
        }
    }

    fn update(&mut self, engine: &mut vesta::Engine) {
        // Update the camera
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

        // TEMP, THIS IS TEMP
        // Used to test how lighting is working
        //let old_position: cgmath::Vector3<_> = self.lights.data.position.into();
        //self.lights.data.position =
        //    cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0))
        //        * old_position;
        engine.renderer.queue.write_buffer(
            &self.lights_uniform.buffer,
            0,
            vesta::bytemuck::cast_slice(&[self.lights_uniform.data]),
        );
    }

    fn render_ui(&mut self, ui: &vesta::imgui::Ui, _engine: &vesta::Engine) {
        let ui_bodies = self.bodies.iter();
        let cam = &self.camera;

        let window = vesta::imgui::Window::new(vesta::imgui::im_str!("Debug"));
        window
            .size([400.0, 700.0], vesta::imgui::Condition::FirstUseEver)
            .build(&ui, || {
                // All bodies
                for b in ui_bodies {
                    let g = ui.begin_group();
                    ui.text(vesta::imgui::im_str!("Body '{}':", b.name));
                    ui.text(vesta::imgui::im_str!("Mass: {:.2} kg", b.mass));
                    ui.text(vesta::imgui::im_str!("Radius: {:.2} m", b.radius));
                    ui.text(vesta::imgui::im_str!(
                        "Velocity: {:.6} m/s",
                        b.velocity.magnitude()
                    ));
                    ui.text(vesta::imgui::im_str!(
                        "Escape Velocity: {:.6} m/s",
                        b.escape_velocity()
                    ));
                    ui.text(vesta::imgui::im_str!(
                        "Position: {:.2}, {:.2}, {:.2}",
                        b.position.x,
                        b.position.y,
                        b.position.z
                    ));

                    ui.spacing();
                    ui.separator();
                    ui.spacing();

                    g.end(&ui);
                }

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

    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        _engine: &vesta::Engine,
    ) {
        // General
        render_pass.set_pipeline(&self.render_pipeline);

        // Render bodies
        render_pass.set_pipeline(&self.c_body_pipeline);
        render_pass.set_bind_group(1, &self.camera.uniform_buffer.bind_group, &[]);
        render_pass.set_bind_group(3, &self.lights_uniform.bind_group, &[]);

        for body in self.bodies.iter() {
            render_pass.set_bind_group(0, &body.texture.bind_group.as_ref().unwrap(), &[]);
            render_pass.set_bind_group(2, &body.uniform_buffer.bind_group, &[]);
            render_pass.draw_mesh(&body.mesh);
        }
    }

    fn resize(&mut self, size: vesta::winit::dpi::PhysicalSize<u32>, _engine: &vesta::Engine) {
        // The screen projection needs to be updated
        self.camera.projection.resize(size.width, size.height);
    }
}
