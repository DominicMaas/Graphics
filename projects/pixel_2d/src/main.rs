mod chunk;
mod input;
mod pixel;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use pixel::{Pixel, PixelType};

/// Tag for the main camera so we can find it during input processing
#[derive(Component)]
pub struct MainCamera;

/// Represents current game state
pub struct WorldState {
    brush_size: i32,
    brush_type: PixelType,
    selected_pixel: Option<Pixel>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            brush_size: 10,
            brush_type: Default::default(),
            selected_pixel: None,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 191.0 / 255.0, 1.0)))
        .insert_resource(WindowDescriptor {
            title: "Pixel 2D".to_string(),
            width: 1920.,
            height: 1080.,

            ..Default::default()
        })
        .init_resource::<WorldState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_startup_system(chunk::setup_chunks)
        .add_system(process_ui)
        .add_system(chunk::update_chunk_textures_system)
        .add_system(chunk::update_chunks)
        .add_system(input::input_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn()
        .insert_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.15,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -42.0, 0.0),
            ..Default::default()
        })
        .insert(MainCamera);
}

fn process_ui(mut egui_context: ResMut<EguiContext>, mut world_state: ResMut<WorldState>) {
    egui::Window::new("Toolbox").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Pixel 2D");
        ui.label("Created by Dominic Maas");
        ui.separator();

        ui.label("Brush Size:");
        ui.add(egui::Slider::new(&mut world_state.brush_size, 1..=100));

        ui.add_space(5.0);
        ui.label("Brush Type:");
        ui.radio_value(&mut world_state.brush_type, PixelType::Air, "Air");
        ui.radio_value(&mut world_state.brush_type, PixelType::Snow, "Snow");
        ui.radio_value(&mut world_state.brush_type, PixelType::Water, "Water");
        ui.radio_value(&mut world_state.brush_type, PixelType::Sand, "Sand");
        ui.radio_value(&mut world_state.brush_type, PixelType::Ground, "Ground");

        match world_state.selected_pixel {
            Some(pixel) => {
                ui.separator();
                ui.heading("Selected Pixel");
                ui.label(format!("Type: {:?}", pixel.get_type()));
                ui.label(format!(
                    "Color: {},{},{}",
                    pixel.get_color().r,
                    pixel.get_color().g,
                    pixel.get_color().b
                ));
            }
            None => {}
        }
    });
}
