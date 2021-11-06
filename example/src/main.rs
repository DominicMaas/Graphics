mod app;
mod cube;

use app::*;
use vesta::winit::dpi::PhysicalSize;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Get log events
        env_logger::init();
    }

    // Config for the engine
    let config = vesta::Config {
        window_title: "Vesta Example".to_string(),
        window_size: PhysicalSize::new(1920, 1080),
    };

    // Create for App, and pass in the config
    vesta::Engine::run::<App>(config);
}
