use bevy::{core::FixedTimestep, prelude::*};

use crate::{celestial_body::CelestialBody, G};

const DT: f32 = 0.01;

pub struct SolarSystemPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    SimulateAcceleration,
    SimulateVelocity,
    SimulateMovement,
}

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second((1.0 / DT) as f64))
                .with_system(
                    simulate_acceleration
                        .system()
                        .label(PhysicsSystem::SimulateAcceleration),
                )
                .with_system(
                    simulate_velocity
                        .system()
                        .label(PhysicsSystem::SimulateVelocity)
                        .after(PhysicsSystem::SimulateAcceleration),
                )
                .with_system(
                    simulate_movement
                        .system()
                        .label(PhysicsSystem::SimulateMovement)
                        .after(PhysicsSystem::SimulateVelocity),
                ),
        );
    }
}

fn simulate_acceleration(mut query: Query<(&Transform, &mut CelestialBody)>) {
    let mut bodies: Vec<(&Transform, Mut<CelestialBody>)> = Vec::new();

    for (transform, mut body) in query.iter_mut() {
        body.acceleration = Vec3::ZERO;

        for (other_transform, other_body) in bodies.iter_mut() {
            let diff = other_transform.translation - transform.translation;

            if let Some(mut force) = diff.try_normalize() {
                let magnitude = G * body.mass * other_body.mass / diff.length_squared();
                force *= magnitude;
                body.acceleration += force;
                other_body.acceleration -= force;
            }
        }

        bodies.push((transform, body));
    }

    for (_, body) in bodies.iter_mut() {
        let m = body.mass;
        body.acceleration /= m;
    }
}

fn simulate_velocity(mut query: Query<&mut CelestialBody>) {
    for mut body in query.iter_mut() {
        let a = body.acceleration;

        body.velocity += a * DT;
    }
}

fn simulate_movement(mut query: Query<(&mut Transform, &CelestialBody)>) {
    for (mut transform, body) in query.iter_mut() {
        transform.translation += body.velocity * DT;
    }
}
