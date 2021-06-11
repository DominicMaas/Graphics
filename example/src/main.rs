mod app;
mod cube;

use app::*;
use futures::executor::block_on;
use vesta::winit::dpi::PhysicalSize;

fn main() {
    // Get log events
    env_logger::init();

    // Config for the engine
    let config = vesta::Config {
        window_title: "Vesta Example".to_string(),
        window_size: PhysicalSize::new(1920, 1080),
    };

    // Unable to run async in main, so block the async,
    // create for App, and pass in the config
    block_on(vesta::Engine::run::<App>(config));
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        super::main();
    }
}
