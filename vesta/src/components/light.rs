use super::Component;
use cgmath::{Deg, Vector3};

/// Light component (added to a game object)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    /// The type of light (also specified light specific features)
    pub light_type: LightType,

    /// The color being emitted by the light
    pub color: Vector3<f32>,

    /// The brightness of the light (multiplied by the light color)
    pub intensity: f32,
}

/// Different possible types of lights
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightType {
    Directional,
    Spot {
        /// Specifies how far the light is emitted from the center of the object
        range: f32,
        /// The angle in degrees at the base of the lights cone
        angle: Deg<f32>,
    },
    Point {
        /// Specifies how far the light is emitted from the center of the object
        range: f32,
    },
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightType::Directional,
            color: Vector3::new(1.0, 1.0, 1.0),
            intensity: 1.0,
        }
    }
}

/// Component metadata for the light
impl Component for Light {
    fn get_name() -> &'static str {
        "Light"
    }
}
