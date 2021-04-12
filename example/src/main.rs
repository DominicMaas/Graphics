mod app;

use app::*;
use futures::executor::block_on;

fn main() {
    // Get log events
    env_logger::init();
    
    // Config for the engine
    let config = vesta::Config {
        window_title: "Vesta Example".to_string()
    };
    
    // Unable to run async in main, so block the async,
    // create for App, and pass in the config
    block_on(vesta::Engine::run::<App>(config));
}
