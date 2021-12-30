use crate::{
    c_body::{CBody, CelestialBodySettings, CelestialBodyTerrain},
    c_body_uniform::CelestialBodyPipeline,
};
use vesta::{
    cgmath::{num_traits::FloatConst, InnerSpace, Vector3},
    components::GameObject,
    winit::event::{MouseButton, VirtualKeyCode},
    LightUniform,
};

pub struct App {
    render_pipeline: vesta::wgpu::RenderPipeline,
    c_body_pipeline: CelestialBodyPipeline,
    camera: vesta::Camera,
    camera_controller: vesta::FpsCameraController,
    bodies: Vec<CBody>,
    lights_uniform: vesta::UniformBuffer<LightUniform>,
    render_wire_frame: bool,
}

impl vesta::VestaApp for App {
    fn init(engine: &mut vesta::Engine) -> Self {
        // Shader strings (todo: in the future load these in as resources or something)
        let main_shader_src = include_str!("shaders/main.wgsl");

        let c_body_pipeline = CelestialBodyPipeline::new(engine);

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
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX | vesta::wgpu::ShaderStages::FRAGMENT,
                            &engine.renderer.device,
                        ),
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "Main Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(main_shader_src.into()))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        // Setup the main camera
        let camera = vesta::CameraBuilder::new()
            .with_position((3450.0, 0.0, 0.0).into())
            .build(
                vesta::PerspectiveProjection::new(
                    engine.get_window_size().width,
                    engine.get_window_size().height,
                    vesta::cgmath::Rad(70.0 / 180.0 * f32::PI()),
                    0.01,
                    10000.0,
                ),
                &engine.renderer.device,
            );

        let camera_controller = vesta::FpsCameraController::default();

        let lights_uniform = vesta::UniformBuffer::new(
            "Light Uniform Buffer",
            vesta::wgpu::ShaderStages::VERTEX | vesta::wgpu::ShaderStages::FRAGMENT,
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

        let mut bodies = Vec::new();

        let sun = CBody::new(
            "Sun".to_string(),
            1000000.0,
            CelestialBodySettings {
                radius: 512.0,
                temp_k: 5500.0,
                atmosphere_density: 0.0,
                terrain: CelestialBodyTerrain {
                    strength: 0.0,
                    ..Default::default()
                },
            },
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            sun_texture,
            &engine.renderer,
        );

        let earth = CBody::new(
            "Earth".to_string(),
            10000.0,
            CelestialBodySettings {
                radius: 256.0,
                temp_k: 300.0,
                atmosphere_density: 1.0,
                terrain: CelestialBodyTerrain {
                    strength: 0.1,
                    num_layers: 5,
                    base_roughness: 1.82,
                    roughness: 2.31,
                    persistence: 0.37,
                    center: (-10.0, 24.0, 165.0).into(),
                    min_value: 0.73,
                },
            },
            Vector3::new(4000.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -sun.calculate_velocity_at_radius(4000.0)),
            earth_texture,
            &engine.renderer,
        );

        bodies.push(sun);
        bodies.push(earth);

        // Build the meshes for these bodies
        for b in bodies.iter_mut() {
            b.rebuild_mesh(&engine.renderer);
        }

        engine.set_cursor_captured(true);

        Self {
            render_pipeline,
            c_body_pipeline,
            camera,
            camera_controller,
            bodies,
            lights_uniform,
            render_wire_frame: true,
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
                let sqr_distance: f32 =
                    (body2.transform.position - body.transform.position).magnitude2();
                let force_direction: Vector3<f32> =
                    (body2.transform.position - body.transform.position).normalize();
                let force: Vector3<f32> =
                    force_direction * body.standard_gravitational_parameter() * body2.mass
                        / sqr_distance;
                let acceleration: Vector3<f32> = force / body.mass;

                body.velocity += acceleration;
            }

            // Run simulations
            body.update(dt);
            engine.renderer.write_uniform_buffer(&body.uniform_buffer);
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

        engine.renderer.write_uniform_buffer(&self.lights_uniform);
    }

    fn render_ui(&mut self, ctx: &vesta::egui::CtxRef, engine: &vesta::Engine) {
        let ui_bodies = self.bodies.iter_mut();
        let cam = &self.camera;
        let render_wire_frame = &mut self.render_wire_frame;

        vesta::egui::Window::new("Debug").show(&ctx, |ui| {
            ui.checkbox(render_wire_frame, "Render Wireframe");

            ui.separator();

            ui.heading("Camera");
            ui.label(format!(
                "Position: {:.2}, {:.2}, {:.2}",
                cam.position.x, cam.position.y, cam.position.z
            ));
            ui.label(format!("Pitch: {:.2}", cam.pitch.0));
            ui.label(format!("Yaw: {:.2},", cam.yaw.0));

            ui.separator();

            ui.heading("Bodies");
            for b in ui_bodies {
                ui.collapsing(format!("{}", b.name), |ui| {
                    ui.label(format!("Mass: {:.2} kg", b.mass));
                    ui.label(format!("Radius: {:.2} m", b.settings.radius));
                    ui.label(format!("Velocity: {:.6} m/s", b.velocity.magnitude()));
                    ui.label(format!("Escape Velocity: {:.6} m/s", b.escape_velocity()));
                    ui.label(format!(
                        "Position: {:.2}, {:.2}, {:.2}",
                        b.transform.position.x, b.transform.position.y, b.transform.position.z
                    ));

                    // A helper function for building the terrain generator functions
                    let mut build_generator_settings = || {
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Strength: ");
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.strength)
                                    .speed(0.01),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Num Layers: ");
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.num_layers)
                                    .speed(1),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Base Roughness: ");
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.base_roughness)
                                    .speed(0.01),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Roughness: ");
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.roughness)
                                    .speed(0.01),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Persistence: ");
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.persistence)
                                    .speed(0.01),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Center (xyz): ");
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.center.x)
                                    .speed(1),
                            );
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.center.y)
                                    .speed(1),
                            );
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.center.z)
                                    .speed(1),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Min Value: ");
                            ui.add(
                                vesta::egui::DragValue::new(&mut b.settings.terrain.min_value)
                                    .speed(0.01),
                            );
                        });

                        if ui.button("Rebuild Terrain").clicked() {
                            b.rebuild_mesh(&engine.renderer);
                        }

                        ui.separator();
                    };

                    build_generator_settings();
                });
            }
        });
    }

    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        engine: &vesta::Engine,
    ) {
        // General
        render_pass.set_pipeline(&self.render_pipeline);

        // Render bodies
        if self.render_wire_frame {
            render_pass.set_pipeline(&self.c_body_pipeline.outline_render_pipeline);
        } else {
            render_pass.set_pipeline(&self.c_body_pipeline.render_pipeline);
        }

        render_pass.set_bind_group(1, &self.camera.uniform_buffer.bind_group, &[]);
        render_pass.set_bind_group(3, &self.lights_uniform.bind_group, &[]);

        for body in self.bodies.iter_mut() {
            body.render(render_pass, engine, &self.camera);
        }
    }

    fn resize(&mut self, size: vesta::winit::dpi::PhysicalSize<u32>, _engine: &vesta::Engine) {
        // The screen projection needs to be updated
        self.camera.projection.resize(size.width, size.height);
    }
}
