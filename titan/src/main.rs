mod chunk;
mod table;
mod terrain;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_rapier3d::prelude::*;
use chunk::chunk_setup;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};
use terrain::Terrain;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample8)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(ClearColor(Color::rgb(0.5294, 0.8078, 0.9216)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.15,
        })
        .insert_resource(Terrain::new(rand::random::<u64>()))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Infinite Drive - Dominic Maas".to_string(),
                        resolution: (1920.0, 1080.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugin(EguiPlugin)
        .add_plugin(AtmospherePlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(chunk_setup)
        .add_system(process_ui)
        .run();
}

fn setup(
    //mut atmosphere: ResMut<Atmosphere>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    let sun_val: f32 = 2.7;
    let sun_pos = Vec3::new(0.0, sun_val.sin(), sun_val.cos());

    //atmosphere.sun_position = sun_pos;

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_x(-sun_pos.y.atan2(sun_pos.z)),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            num_cascades: 5,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    /*commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial::from(Color::rgb(0.8, 0.2, 0.2))),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial::from(Color::rgb(0.2, 0.2, 0.8))),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 34.0, 1.0)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(StandardMaterial::from(Color::rgb(1.0, 1.0, 1.0))),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(1.0, 36.0, 0.0)));*/

    // Camera
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(0.0, 32.0, 5.0),
            Vec3::new(0., 32.0, 0.),
            Vec3::Y,
        ))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 1.0,
                rings: 0,
                depth: 2.0,
                latitudes: 16,
                longitudes: 32,
                uv_profile: shape::CapsuleUvProfile::Aspect,
            })),
            material: materials.add(StandardMaterial::from(Color::rgb(0.0, 0.0, 0.0))),
            ..Default::default()
        })
        .insert(AtmosphereCamera::default());

    //.insert(RigidBody::KinematicPositionBased)
    //.insert(Collider::capsule_y(1.0, 1.0))
    //.insert(LockedAxes::ROTATION_LOCKED)
    //.insert(Ccd::enabled())
    //.insert(AtmosphereCamera(None));
}

fn process_ui(mut contexts: EguiContexts, mut atmosphere: AtmosphereMut<Nishita>) {
    egui::Window::new("Infinite Drive").show(contexts.ctx_mut(), |ui| {
        ui.label("Created by Dominic Maas");
        ui.separator();

        ui.label("Sun Position: ");
        ui.add(egui::Slider::new(&mut atmosphere.sun_position.x, 0.0..=1.0));
        ui.add(egui::Slider::new(&mut atmosphere.sun_position.y, 0.0..=1.0));
        ui.add(egui::Slider::new(&mut atmosphere.sun_position.z, 0.0..=1.0));
    });
}
