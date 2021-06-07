use std::time::Instant;

use crate::{VestaApp, config::Config, io::{IO, Keyboard, Mouse}, renderer::Renderer, texture};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

struct GUI {
    gui_context: imgui::Context,
    gui_platform: imgui_winit_support::WinitPlatform,
    gui_renderer: imgui_wgpu::Renderer,
}

pub struct Engine {
    window: Window,
    pub io: IO,
    pub renderer: Renderer,
    window_size: winit::dpi::PhysicalSize<u32>,
    cursor_captured: bool,
    // Timing
    delta_time: f32,
    current_time: Instant,
    accumulator: f32,
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
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        // Create a depth texture
        let depth_texture =
            texture::Texture::create_depth(&device, &swap_chain_desc, Some("Depth Texture"))
                .unwrap();

        // -------------- GUI ------------------ //

        // Setup ImGUI and attach it to our window, ImGui is used as the GUI for this
        // application
        let mut gui_context = imgui::Context::create();
        let mut gui_platform = imgui_winit_support::WinitPlatform::init(&mut gui_context);
        gui_platform.attach_window(
            gui_context.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        gui_context.set_ini_filename(None);

        // Setup the font for ImGui
        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        gui_context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        gui_context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        let renderer_config = imgui_wgpu::RendererConfig {
            texture_format: swap_chain_desc.format,
            ..Default::default()
        };

        let gui_renderer =
            imgui_wgpu::Renderer::new(&mut gui_context, &device, &queue, renderer_config);

        // Renderer information, this will be sent to the app implementation so it can access resources
        let renderer = Renderer {
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            depth_texture,
        };
        let mut gui = GUI {
            gui_context,
            gui_platform,
            gui_renderer,
        };
        let mut engine = Engine {
            window,
            io: IO {
                keyboard: Keyboard::new(),
                mouse: Mouse::new()
            },
            renderer,
            window_size,
            cursor_captured: false,
            delta_time: 0.01,
            current_time: Instant::now(),
            accumulator: 0.0,
        };

        // First initllize all the apps resources (shaders, pipelines etc.)
        let mut app = V::init(&mut engine);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            // Handle gui events
            let io = gui.gui_context.io_mut();
            gui.gui_platform.handle_event(io, &engine.window, &event);

            engine.handle_events(event, control_flow, &mut app, &mut gui);
        });
    }

    fn handle_events<V: VestaApp>(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
        app: &mut V,
        gui: &mut GUI,
    ) {
        match event {
            Event::RedrawRequested(_) => {
                // Timing logic
                let new_time = Instant::now();
                let frame_time = new_time - self.current_time;

                self.current_time = new_time;
                self.accumulator += frame_time.as_secs_f32();

                while self.accumulator >= self.delta_time {
                    gui.gui_context
                        .io_mut()
                        .update_delta_time(std::time::Duration::from_secs_f32(self.delta_time));
                    app.physics_update(self.delta_time, self);

                    self.accumulator -= self.delta_time;
                }

                // Run the frame update
                app.update(self);
                
                // Perform the actual rendering
                match self.render(gui, app) {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => {
                        self.resize(app, self.window_size);
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::NewEvents (_) => {
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
                /*match event {
                    WindowEvent::Resized(_) => {}
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested => {}
                    WindowEvent::Destroyed => {}
                    WindowEvent::DroppedFile(_) => {}
                    WindowEvent::HoveredFile(_) => {}
                    WindowEvent::HoveredFileCancelled => {}
                    WindowEvent::ReceivedCharacter(_) => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {}
                    WindowEvent::ModifiersChanged(_) => {}
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {}
                    WindowEvent::CursorEntered { device_id } => {}
                    WindowEvent::CursorLeft { device_id } => {}
                    WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {}
                    WindowEvent::MouseInput { device_id, state, button, modifiers } => {}
                    WindowEvent::TouchpadPressure { device_id, pressure, stage } => {}
                    WindowEvent::AxisMotion { device_id, axis, value } => {}
                    WindowEvent::Touch(_) => {}
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {}
                    WindowEvent::ThemeChanged(_) => {}
                }*/
                
                // Handle mouse and keyboard events
                self.io.mouse.handle_event(event);
                self.io.keyboard.handle_event(event);
                
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => self.resize(app, *physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.resize(app, **new_inner_size),
                    _ => { }
                }
            }
            _ => {}
        }
    }

    fn resize<V: VestaApp>(&mut self, app: &mut V, new_size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = new_size;

        self.renderer.swap_chain_desc.width = new_size.width;
        self.renderer.swap_chain_desc.height = new_size.height;

        self.renderer.swap_chain = self
            .renderer
            .device
            .create_swap_chain(&self.renderer.surface, &self.renderer.swap_chain_desc);

        self.renderer.depth_texture = self
            .renderer
            .create_depth_texture(Some("Depth Texture"))
            .unwrap();

        app.resize(new_size, self);
    }

    fn render<V: VestaApp>(&self, gui: &mut GUI, app: &mut V) -> Result<(), wgpu::SwapChainError> {
        // Prepare the UI
        gui.gui_platform
            .prepare_frame(gui.gui_context.io_mut(), &self.window)
            .expect("Failed to prepare frame!");

        // Get a frame
        let frame = self.renderer.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .renderer
            .device
            .create_command_encoder(&Default::default());

        let ui = gui.gui_context.frame();
        {
            app.render_ui(&ui, &self);
        }

        // ---- MAIN ---- //
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
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

            app.render(&mut render_pass, &self)
        }

        // ---- UI ---- //
        {
            let mut ui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            // Render the UI
            gui.gui_platform.prepare_render(&ui, &self.window);
            gui.gui_renderer
                .render(
                    ui.render(),
                    &self.renderer.queue,
                    &self.renderer.device,
                    &mut ui_pass,
                )
                .expect("Failed to render UI!");
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
