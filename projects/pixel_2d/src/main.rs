use bevy::prelude::*;

//use crate::app::App;
//use vesta::winit::dpi::PhysicalSize;

//mod app;
//mod chunk;
//mod pixel;
//mod world;

fn main() {
    // Config for the engine
    //let config = vesta::Config {
    //    window_title: "Pixel 2D".to_string(),
    //    window_size: PhysicalSize::new(1920, 1080),
    //};

    // Create for App, and pass in the config
    //vesta::Engine::run::<App>(config);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Pixel 2D".to_string(),
            width: 1920.,
            height: 1080.,

            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
