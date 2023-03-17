mod chunk;
mod player;

use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_particle_systems::*;
use bevy_pixel_camera::*;
use bevy_rapier2d::prelude::*;

use bracket_random::prelude::RandomNumberGenerator;
use chunk::{ChunkResources, SpawnChunkEvent, CHUNK_X};
use player::Player;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

/// Everything is scaled by this value
pub const GAME_SCALE: f32 = 50.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ALICE_BLUE))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            GAME_SCALE,
        ))
        .add_plugin(PixelCameraPlugin)
        .add_plugin(ParticleSystemPlugin::default())
        .add_event::<SpawnChunkEvent>()
        .add_startup_system(setup)
        .add_system(player::player_animation_system)
        .add_system(player::player_movement_system)
        .add_system(player::player_camera_system)
        .add_system(chunk::spawn_chunk_system)
        .run();
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_chunk: EventWriter<SpawnChunkEvent>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //  let background_texture_handle = asset_server.load("01_background.png");

    // Clouds, TODO: Move to weather system
    commands
        // Add the bundle specifying the particle system itself.
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 20,
                texture: ParticleTexture::Sprite(asset_server.load("cloud.png")),

                spawn_rate_per_second: 1.0.into(),
                initial_speed: JitteredValue::jittered(50.0, -25.0..25.0),
                scale: ValueOverTime::Constant(10.0),
                lifetime: 1000.0.into(),
                emitter_shape: Line {
                    length: 64.0 * 50.0,
                    angle: 0.0.into(),
                }
                .into(),

                looping: true,
                system_duration_seconds: 10.0,
                z_value_override: Some(JitteredValue::new(-0.1)),
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(0.0, 32.0 * GAME_SCALE, -10.0),
            ..ParticleSystemBundle::default()
        })
        // Add the playing component so it starts playing. This can be added later as well.
        .insert(Playing);

    let mut pixel_camera = PixelCameraBundle::from_zoom(5);
    pixel_camera.camera.hdr = true;

    commands.spawn((
        pixel_camera,
        BloomSettings::default(),
        MainCamera,
        Tonemapping::BlenderFilmic,
        DebandDither::Enabled,
    ));

    // Player
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(8.0, 11.0),
        KinematicCharacterController::default(),
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
        Friction::default(),
        Ccd::enabled(),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: player::PLAYER_ANIM_IDLE,
                color: Color::rgb(1.1, 1.1, 1.1),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 32.0 * GAME_SCALE, 0.0),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player::new(),
    ));

    //  Test Ground
    let texture_handle = asset_server.load("sheet_2.png");
    let terrain_material = materials.add(ColorMaterial::from(texture_handle));

    let mut rng = RandomNumberGenerator::new();

    commands.insert_resource(ChunkResources {
        material: terrain_material,
        tile_size: Vec2::new(16.0, 16.0),
        columns: 17,
        rows: 8,
        seed: rng.next_u64(),
    });

    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * -4.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * -3.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * -2.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * -1.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((0.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * 1.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * 2.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * 3.0, 0.0).into()));
    ev_spawn_chunk.send(SpawnChunkEvent((CHUNK_X as f32 * 4.0, 0.0).into()));
}
