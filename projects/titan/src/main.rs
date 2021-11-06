mod app;
mod cube;
mod entities;
mod sky_shader;
mod world;

use app::App;

fn main() {
    // Config for the engine
    let config = vesta::Config {
        window_title: "Project Titan".to_string(),
        window_size: (1920, 1080).into(),
    };

    // Create for App, and pass in the config
    vesta::Engine::run::<App>(config);
}
