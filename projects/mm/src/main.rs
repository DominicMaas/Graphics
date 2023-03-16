mod chunk;
mod player;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::{
        camera::ScalingMode,
        mesh::Indices,
        render_resource::PrimitiveTopology,
        settings::{WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_particle_systems::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyType};

use chunk::{Chunk, ChunkBundle};
use player::Player;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ALICE_BLUE))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(ParticleSystemPlugin::default())
        .add_startup_system(setup)
        .add_system(player::player_animation_system)
        .add_system(player::player_movement_system)
        .add_system(player::player_camera_system)
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
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Clouds, TODO: Move to weather system
    commands
        // Add the bundle specifying the particle system itself.
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 20,
                texture: ParticleTexture::Sprite(asset_server.load("cloud.png")),
                spawn_rate_per_second: 1.0.into(),
                initial_speed: JitteredValue::jittered(15.0, -5.0..5.0),
                scale: ValueOverTime::Constant(10.0),
                lifetime: 1000.0.into(),
                emitter_shape: Line {
                    length: 400.0,
                    angle: 0.0.into(),
                }
                .into(),

                looping: true,
                system_duration_seconds: 10.0,
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(-20.0, 150.0, -10.0),
            ..ParticleSystemBundle::default()
        })
        // Add the playing component so it starts playing. This can be added later as well.
        .insert(Playing);

    // Camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                scale: 1000.0,
                scaling_mode: ScalingMode::FixedVertical(1.),
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
        MainCamera,
    ));

    // Player
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(8.0, 11.0),
        KinematicCharacterController::default(),
        Velocity::zero(),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: player::PLAYER_ANIM_IDLE,
                color: Color::rgb(1.5, 1.5, 1.5),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 0.0).with_scale(Vec3::splat(5.0)),

            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player::new(),
        LockedAxes::ROTATION_LOCKED_X
            | LockedAxes::ROTATION_LOCKED_Y
            | LockedAxes::ROTATION_LOCKED_Z,
    ));

    //  Test Ground
    let texture_handle = asset_server.load("Untitled.png");
    let terrain_material = materials.add(ColorMaterial::from(texture_handle));

    let mut c = Chunk::default();
    c.set_tile(0, 15, chunk::TileType::Air);
    c.set_tile(1, 15, chunk::TileType::Air);
    c.set_tile(1, 14, chunk::TileType::Air);
    c.set_tile(2, 13, chunk::TileType::Air);
    c.set_tile(2, 13, chunk::TileType::Air);
    c.set_tile(2, 14, chunk::TileType::Air);
    c.set_tile(2, 14, chunk::TileType::Air);
    c.set_tile(2, 15, chunk::TileType::Air);
    c.set_tile(2, 15, chunk::TileType::Air);
    c.set_tile(3, 15, chunk::TileType::Air);
    c.set_tile(3, 14, chunk::TileType::Air);

    let m = c.create_mesh();

    commands.spawn((
        ChunkBundle {
            chunk: c.clone(),
            material: MaterialMesh2dBundle {
                mesh: meshes.add(m).into(),
                transform: Transform::from_xyz(0.0, 16.0 * -100.0, 0.0)
                    .with_scale(Vec3::splat(100.0)),
                material: terrain_material.clone(),
                ..default()
            },
        },
        c.create_colider_mesh(),
        //Collider::cuboid(10.0, 10.0),
        RigidBody::Fixed,
    ));
}
