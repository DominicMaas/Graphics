mod app;
mod c_body;
mod terrain_face;
mod utils;

use app::App;

fn main() {
    // Config for the engine
    let config = vesta::Config {
        window_title: "Eris".to_string(),
        window_size: (1920, 1080).into(),
    };

    // Create for App, and pass in the config
    vesta::Engine::run::<App>(config);
}
