mod app;

use vesta::{config::Config, engine::Engine};
use futures::executor::block_on;

fn main() {
    env_logger::init();
    
    let app = app::App::new();
    
    let config = Config {
        window_title: "Vesta Example".to_string()
    };
    
    block_on(Engine::run(config, app));
}
