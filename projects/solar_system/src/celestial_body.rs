use bevy::prelude::*;

use crate::G;

#[derive(Bundle)]
pub struct CelestialBodyBundle {
    pub info: CelestialBody,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for CelestialBodyBundle {
    fn default() -> Self {
        Self {
            info: Default::default(),
            mesh: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct CelestialBody {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl Default for CelestialBody {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mass: Default::default(),
            radius: Default::default(),
            velocity: Default::default(),
            acceleration: Default::default(),
        }
    }
}

impl CelestialBody {
    pub fn standard_gravitational_parameter(&self) -> f32 {
        G * self.mass
    }

    pub fn calculate_velocity_at_radius(&self, radius: f32) -> f32 {
        (self.standard_gravitational_parameter() / radius).sqrt()
    }

    pub fn escape_velocity(&self) -> f32 {
        let n = 2.0 * self.standard_gravitational_parameter();
        let d = self.radius;
        let nd = n / d;

        nd.sqrt()
    }
}
