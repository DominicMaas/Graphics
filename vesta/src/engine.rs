use std::time::Instant;

use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};
use crate::{VestaApp, config::Config, renderer::Renderer, texture};

struct GUI {
    gui_context: imgui::Context,
    gui_platform: imgui_winit_support::WinitPlatform, 
    gui_renderer: imgui_wgpu::Renderer,
}

pub struct Engine {
    pub renderer: Renderer,
    pub window_size: winit::dpi::PhysicalSize<u32>
}

impl Engine {
    pub async fn run<V: VestaApp + 'static>(config: Config)  {        
        // Loop that will run all the events
        let event_loop = EventLoop::new();
         
        // Build the window with specified config
        let window = WindowBuilder::new()
            .with_title(config.window_title)
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
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
    
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
        
        // Create a depth texture
        let depth_texture =
            texture::Texture::create_depth(&device, &swap_chain_desc, Some("Depth Texture")).unwrap();
        
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
        gui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData {
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

        let gui_renderer = imgui_wgpu::Renderer::new(&mut gui_context, &device, &queue, renderer_config);
            
        // Renderer information, this will be sent to the app implementation so it can access resources
        let renderer = Renderer { surface, device, queue, swap_chain_desc, swap_chain, depth_texture };
        let mut gui = GUI { gui_context, gui_platform, gui_renderer };
        let mut engine = Engine { renderer, window_size };
        
        // First initllize all the apps resources (shaders, pipelines etc.)
        let mut app = V::init(&engine);
                
        // Update timings
        //let mut t: f64 = 0.0;
        const DT: f32 = 0.01;
                
        let mut current_time = Instant::now();
        let mut accumulator: f32 = 0.0;
                
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            
            // Handle gui events
            let io = gui.gui_context.io_mut();
            gui.gui_platform.handle_event(io, &window, &event);
            
            match event {
                Event::RedrawRequested(_) => {
                    // Timing logic
                    let new_time = Instant::now();
                    let frame_time = new_time - current_time;
                    
                    current_time = new_time;
                    
                    accumulator += frame_time.as_secs_f32();
                    
                    while accumulator >= DT {
                        gui.gui_context.io_mut().update_delta_time(std::time::Duration::from_secs_f32(DT));
                        app.update(DT, &engine);
                        
                        accumulator -= DT;
                        //t += DT;
                    }
                    
                    // Perform the actual rendering
                    match Self::render(&window, &engine, &mut gui, &mut app) {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SwapChainError::Lost) => {
                            let size = engine.window_size;
                            Self::resize(&mut engine, &mut app, size)
                        },
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::DeviceEvent { ref event, .. } => {
                    app.device_input(event, &engine);
                }
                Event::WindowEvent { ref event, .. } => {
                    if !app.input(event, &engine) {
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                Self::resize(&mut engine, &mut app, *physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                Self::resize(&mut engine, &mut app, **new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        });
    }
    
    fn resize<V: VestaApp>(engine: &mut Engine, app: &mut V, new_size: winit::dpi::PhysicalSize<u32>) {
        engine.window_size = new_size;
        
        engine.renderer.swap_chain_desc.width = new_size.width;
        engine.renderer.swap_chain_desc.height = new_size.height;
        
        engine.renderer.swap_chain = engine.renderer.device.create_swap_chain(&engine.renderer.surface, &engine.renderer.swap_chain_desc);
        
        engine.renderer.depth_texture = engine.renderer.create_depth_texture(Some("Depth Texture")).unwrap();
        
        app.resize(new_size, engine);
    }
    
    fn render<V: VestaApp>(window: &Window, engine: &Engine, gui: &mut GUI, app: &mut V) -> Result<(), wgpu::SwapChainError> {
        // Prepare the UI
        gui.gui_platform
            .prepare_frame(gui.gui_context.io_mut(), &window)
            .expect("Failed to prepare frame!");
        
        // Get a frame
        let frame = engine.renderer.swap_chain.get_current_frame()?.output;
        let mut encoder = engine.renderer.device.create_command_encoder(&Default::default());
        
        let ui = gui.gui_context.frame();
        {
            app.render_ui(&ui, engine);
        }
        
        // ---- MAIN ---- //
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &engine.renderer.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            
            app.render(&mut render_pass, engine)
        }
        
        // ---- UI ---- //
        {
            let mut ui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            // Render the UI
            gui.gui_platform.prepare_render(&ui, &window);
            gui.gui_renderer
                .render(ui.render(), &engine.renderer.queue, &engine.renderer.device, &mut ui_pass)
                .expect("Failed to render UI!");
        }
        
        // Finished with the frame
        engine.renderer.queue.submit(std::iter::once(encoder.finish()));  
        Ok(())
    }
}
