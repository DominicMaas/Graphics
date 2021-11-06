use crate::app::App;
use vesta::winit::dpi::PhysicalSize;

mod app;
mod chunk;
mod pixel;
mod world;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Get log events
        env_logger::init();
    }

    // Config for the engine
    let config = vesta::Config {
        window_title: "Pixel 2D".to_string(),
        window_size: PhysicalSize::new(1920, 1080),
    };

    // Create for App, and pass in the config
    vesta::Engine::run::<App>(config);
}
