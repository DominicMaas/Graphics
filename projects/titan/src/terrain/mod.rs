use bevy::{asset::HandleId, prelude::*};

use crate::terrain::chunk::{create_mesh, Chunk, ChunkBundle};

mod chunk;
mod generator;

#[derive(Component)]
pub struct Terrain {
    generator: self::generator::Generator,
    chunk_material_id: HandleId,
}

// ----- TerrainPlugin ----- //

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(terrain_system_startup)
            .add_system(terrain_system);
    }
}

// Prepares the terrain system, sets up generators etc.
fn terrain_system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let chunk_material_id = materials
        .add(StandardMaterial {
            base_color_texture: Some(asset_server.load("block_map.png")),
            ..Default::default()
        })
        .id;

    commands.insert_resource(Terrain {
        generator: self::generator::Generator::new(456456456345634),
        chunk_material_id,
    });

    commands
        .spawn()
        .insert_bundle(ChunkBundle::new(meshes.add(create_mesh()), materials));

    //commands.spawn_bundle(ChunkBundle { c });

    println!("Startup!");

    // TEMP
}

// Terrain system running on loop. Used to load, generate and unload chunks throughout the game
fn terrain_system() {}
