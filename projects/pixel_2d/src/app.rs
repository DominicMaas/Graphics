use crate::pixel::PixelType;
use crate::world::World;
use vesta::cgmath::Vector2;
use vesta::cgmath::Vector3;
use vesta::imgui;
use vesta::imgui::im_str;
use vesta::wgpu::RenderPass;
use vesta::winit::dpi::PhysicalSize;
use vesta::winit::event::{MouseButton, VirtualKeyCode};
use vesta::Engine;

pub struct App {
    pixel_pipeline: vesta::wgpu::RenderPipeline,
    camera: vesta::Camera,
    camera_controller: vesta::CameraController,
    world: World,
    brush_size: i32,
    brush_type: PixelType
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
            (0.0, 0.0, 100.0).into(),
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
            brush_size: 1,
            brush_type: PixelType::Water
        }
    }
    
    fn update(&mut self, engine: &mut vesta::Engine) {  
        self.camera_controller.process_input(&engine.io);    
        self.camera_controller.update_camera(&mut self.camera, &engine, false);
        
        self.camera.update_uniforms(&engine.renderer);
        
        if engine.io.mouse.get_button(MouseButton::Left) {
            let pos = engine.io.mouse.get_position_f32();
            let world_pos = self.camera.screen_to_world_point(Vector3::new(pos.x, pos.y, 0.0001));
            
            self.world.paint(self.brush_type, self.brush_size, Vector2::new(world_pos.x, world_pos.y));
        }
        
        if engine.io.keyboard.get_key_down(VirtualKeyCode::R) {
            println!("Adding snow...");
            self.world.add_snow();
        }
        
        self.world.update();
        self.world.rebuild(&engine.renderer);
    }

    fn render_ui(&mut self, ui: &imgui::Ui, _engine: &Engine) {
        let window = vesta::imgui::Window::new(im_str!("Toolbox"));
        window
            .size([400.0, 700.0], vesta::imgui::Condition::FirstUseEver)
            .build(&ui, || {
                
                let cg = ui.begin_group();
                ui.input_int(im_str!("Brush Size"), &mut self.brush_size)
                    .build();
                    
                ui.radio_button(im_str!("Brush: Air"), &mut self.brush_type, PixelType::Air);
                ui.radio_button(im_str!("Brush: Ground"), &mut self.brush_type, PixelType::Ground);
                ui.radio_button(im_str!("Brush: Snow"), &mut self.brush_type, PixelType::Snow);
                ui.radio_button(im_str!("Brush: Water"), &mut self.brush_type, PixelType::Water);
                    
                cg.end(&ui);
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
