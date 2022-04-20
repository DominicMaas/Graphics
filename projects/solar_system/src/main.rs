mod celestial_body;
mod solar_system_plugin;

use bevy::prelude::*;
use celestial_body::{CelestialBody, CelestialBodyBundle};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};
use solar_system_plugin::SolarSystemPlugin;

/// This custom universe uses this G
pub const G: f32 = 6.67430e-11_f32;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Solar System".to_string(),
            width: 1920.,
            height: 1080.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(SolarSystemPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_info = CelestialBody {
        name: "Sun".to_string(),
        mass: 100000000000.0,
        radius: 6.0,
        velocity: (0.0, 0.0, 0.0).into(),
        ..Default::default()
    };

    let earth_info = CelestialBody {
        name: "Earth".to_string(),
        mass: 100.0,
        radius: 1.0,
        velocity: (0.0, 0.0, -sun_info.calculate_velocity_at_radius(10.0)).into(),
        ..Default::default()
    };

    commands.spawn_bundle(CelestialBodyBundle {
        info: earth_info,
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            subdivisions: 6,
            radius: 1.0,
        })),
        material: materials.add(Color::rgb(0.3, 0.9, 0.3).into()),
        transform: Transform::from_xyz(10.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(CelestialBodyBundle {
        info: sun_info,
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            subdivisions: 6,
            radius: 6.0,
        })),
        material: materials.add(Color::rgb(0.8, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(FpsCameraBundle::new(
        FpsCameraController {
            smoothing_weight: 0.0,
            mouse_rotate_sensitivity: Vec2::splat(0.0007),
            translate_sensitivity: 2.3,
            ..Default::default()
        },
        PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                fov: std::f32::consts::PI / 4.0,
                near: 0.01,
                far: 10000.0,
                aspect_ratio: 1.0,
            },
            ..Default::default()
        },
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0., 0., 0.),
    ));

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
}
