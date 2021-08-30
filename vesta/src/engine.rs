use std::time::Instant;

use crate::{
    config::Config,
    io::{Keyboard, Mouse, IO},
    renderer::Renderer,
    texture, VestaApp,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct Gui {
    platform: egui_winit_platform::Platform,
    renderer: egui_wgpu_backend::RenderPass,
}

pub struct Time {
    frame_delta_time: f32,
    delta_time: f32,
    current_time: Instant,
    start_time: Instant,
    accumulator: f32,
}

impl Time {
    pub fn get_delta_time(&self) -> f32 {
        self.frame_delta_time
    }
}

pub struct Engine {
    window: Window,
    pub io: IO,
    pub renderer: Renderer,
    window_size: winit::dpi::PhysicalSize<u32>,
    cursor_captured: bool,
    // Timing
    pub time: Time,
}

impl Engine {
    pub async fn run<V: VestaApp + 'static>(config: Config) {
        // Loop that will run all the events
        let event_loop = EventLoop::new();

        // Build the window with specified config
        let window = WindowBuilder::new()
            .with_title(config.window_title)
            .with_inner_size(config.window_size)
            .build(&event_loop)
            .unwrap();

        // Determined window size
        let window_size = window.inner_size();

        // New WGPU instance and surface to render on
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };

        // Request a high performance adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // Request a device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor::default(),
                None, // Trace path
            )
            .await
            .unwrap();

        // Configure rendering surface
        let surface_format = surface.get_preferred_format(&adapter).unwrap();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        // Create a depth texture
        let depth_texture =
            texture::Texture::create_depth(&device, &surface_config, Some("Depth Texture"))
                .unwrap();

        // -------------- GUI ------------------ //

        // Create the platform (winit)
        let gui_platform = egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
            physical_width: window_size.width as u32,
            physical_height: window_size.height as u32,
            scale_factor: window.scale_factor(),
            font_definitions: egui::FontDefinitions::default(),
            style: Default::default(),
        });

        // Create the renderer (wgpu)
        let gui_renderer = egui_wgpu_backend::RenderPass::new(&device, surface_format, 1);

        // Renderer information, this will be sent to the app implementation so it can access resources
        let renderer = Renderer {
            surface,
            device,
            queue,
            surface_config,
            depth_texture,
        };

        let mut gui = Gui {
            platform: gui_platform,
            renderer: gui_renderer,
        };

        let mut engine = Engine {
            window,
            io: IO {
                keyboard: Keyboard::new(),
                mouse: Mouse::new(),
            },
            renderer,
            window_size,
            cursor_captured: false,
            time: Time {
                delta_time: 0.01,
                frame_delta_time: 0.0,
                current_time: Instant::now(),
                start_time: Instant::now(),
                accumulator: 0.0,
            },
        };

        // First initialize all the apps resources (shaders, pipelines etc.)
        let mut app = V::init(&mut engine);

        // Trigger a resize straight away to ensure any sizing code is run
        app.resize(window_size, &engine);

        // Run the main event loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            // Handle gui events
            gui.platform.handle_event(&event);

            // Handle engine events
            engine.handle_events(&event, control_flow, &mut app, &mut gui);
        });
    }

    fn handle_events<V: VestaApp>(
        &mut self,
        event: &Event<()>,
        control_flow: &mut ControlFlow,
        app: &mut V,
        gui: &mut Gui,
    ) {
        let gui_capture = gui.platform.captures_event(event);

        match event {
            Event::RedrawRequested(_) => {
                // Update the GUI
                gui.platform.update_time(self.time.start_time.elapsed().as_secs_f64());

                // Timing logic
                let new_time = Instant::now();
                let frame_time = new_time - self.time.current_time;

                self.time.frame_delta_time = frame_time.as_secs_f32();

                self.time.current_time = new_time;
                self.time.accumulator += frame_time.as_secs_f32();

                while self.time.accumulator >= self.time.delta_time {
                    app.physics_update(self.time.delta_time, self);
                    self.time.accumulator -= self.time.delta_time;
                }

                // Run the frame update
                app.update(self);

                // Perform the actual rendering
                match self.render(gui, app) {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        self.resize(app, self.window_size);
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::NewEvents(_) => {
                self.io.mouse.clear_events();
                self.io.keyboard.clear_events();
            }
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            Event::DeviceEvent { ref event, .. } => {
                self.io.mouse.handle_device_event(event);
            }
            Event::WindowEvent { ref event, .. } => {
                // Handle mouse and keyboard events if the UI is not handling them
                if !gui_capture {
                    self.io.mouse.handle_event(event);
                    self.io.keyboard.handle_event(event);
                }

                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => self.resize(app, *physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.resize(app, **new_inner_size)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn resize<V: VestaApp>(&mut self, app: &mut V, new_size: winit::dpi::PhysicalSize<u32>) {
        // Ensure engine size is set correctly
        self.window_size = new_size;

        // Resize the surface
        self.renderer.surface_config.width = new_size.width;
        self.renderer.surface_config.height = new_size.height;
        self.renderer.surface.configure(&self.renderer.device, &self.renderer.surface_config);

        // Recreate the depth texture
        self.renderer.depth_texture = self
            .renderer
            .create_depth_texture(Some("Depth Texture"))
            .unwrap();

        // Run any app specific events
        app.resize(new_size, self);
    }

    fn render<V: VestaApp>(&mut self, gui: &mut Gui, app: &mut V) -> Result<(), wgpu::SurfaceError> {

        // Get a frame and associated view
        let out_frame = self.renderer.surface.get_current_frame()?.output;
        let out_view = out_frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .renderer
            .device
            .create_command_encoder(&Default::default());

        // ---- MAIN ---- //
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &out_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.renderer.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            app.render(&mut render_pass, self)
        }

        // ---- UI ---- //
        {
            // Start rendering frame
            gui.platform.begin_frame();

            // Render app UI
            app.render_ui(&gui.platform.context(), self);

            // End the UI frame. We could now handle the output and draw the UI with the backend.
            let (_output, paint_commands) = gui.platform.end_frame(Some(&self.window));
            let paint_jobs = gui.platform.context().tessellate(paint_commands);

            // Upload all resources for the GPU.
            let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
                physical_width: self.renderer.surface_config.width,
                physical_height: self.renderer.surface_config.height,
                scale_factor: self.window.scale_factor() as f32,
            };

            gui.renderer.update_texture(&self.renderer.device, &self.renderer.queue, &gui.platform.context().texture());
            gui.renderer.update_user_textures(&self.renderer.device, &self.renderer.queue);
            gui.renderer.update_buffers(&mut self.renderer.device, &mut self.renderer.queue, &paint_jobs, &screen_descriptor);

            // Render the UI
            gui.renderer.execute(
                &mut encoder,
                &out_view,
                &paint_jobs,
                &screen_descriptor,
                None,
            ).expect("Failed to render UI!");
        }

        // Finished with the frame
        self.renderer
            .queue
            .submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    /// Gets the current window size
    pub fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_size
    }

    /// Sets if the current cursor is captured
    pub fn set_cursor_captured(&mut self, captured: bool) {
        self.cursor_captured = captured;

        if self.cursor_captured {
            self.window.set_cursor_grab(true).ok();
            self.window.set_cursor_visible(false);
        } else {
            self.window.set_cursor_grab(false).ok();
            self.window.set_cursor_visible(true);
        }
    }

    pub fn is_cursor_captured(&self) -> bool {
        self.cursor_captured
    }
}
