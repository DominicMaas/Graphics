use crate::app::App;
use futures::executor::block_on;
use vesta::winit::dpi::PhysicalSize;

mod app;
mod chunk;
mod world;

fn main() {
    // Get log events
    env_logger::init();

    // Config for the engine
    let config = vesta::Config {
        window_title: "Pixel 2D".to_string(),
        window_size: PhysicalSize::new(1920, 1080),
    };

    // Unable to run async in main, so block the async,
    // create for App, and pass in the config
    block_on(vesta::Engine::run::<App>(config));
}
