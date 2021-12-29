use crate::terrain_face::TerrainFace;
use crate::utils::G;
use bracket_noise::prelude::FastNoise;
use std::time::Duration;
use vesta::cgmath::Vector3;
use vesta::Math;

pub struct CBody {
    pub name: String,
    pub mass: f32,
    pub settings: CelestialBodySettings,
    generator: CelestialBodyTerrainGenerator,
    pub velocity: Vector3<f32>,
    pub transform: vesta::components::Transform,
    pub uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
    pub texture: vesta::Texture,
    pub faces: Vec<TerrainFace>,
    pub resolution: u32,
}

impl CBody {
    pub fn new(
        name: String,
        mass: f32,
        settings: CelestialBodySettings,
        position: Vector3<f32>,
        velocity: Vector3<f32>,
        texture: vesta::Texture,
        renderer: &vesta::Renderer,
    ) -> Self {
        let transform = vesta::components::Transform {
            position,
            ..Default::default()
        };

        let uniform_data = vesta::ModelUniform {
            model: transform.calculate_model_matrix(),
            normal: transform.calculate_normal_matrix(),
        };

        let uniform_buffer = vesta::UniformBuffer::new(
            "C-Body Uniform Buffer",
            vesta::wgpu::ShaderStages::VERTEX,
            uniform_data,
            &renderer.device,
        );

        let resolution = 32;

        // Create meshes!
        let mut faces: Vec<TerrainFace> = Vec::new();
        faces.push(TerrainFace::new(resolution, Vector3::new(0.0, 1.0, 0.0))); // Top
        faces.push(TerrainFace::new(resolution, Vector3::new(0.0, -1.0, 0.0))); // Bottom
        faces.push(TerrainFace::new(resolution, Vector3::new(1.0, 0.0, 0.0))); // Left
        faces.push(TerrainFace::new(resolution, Vector3::new(-1.0, 0.0, 0.0))); // Right
        faces.push(TerrainFace::new(resolution, Vector3::new(0.0, 0.0, 1.0))); // Front?
        faces.push(TerrainFace::new(resolution, Vector3::new(0.0, 0.0, -1.0))); // Back?

        Self {
            name,
            mass,
            settings,
            generator: CelestialBodyTerrainGenerator::new(),
            velocity,
            transform,
            uniform_buffer,
            texture,
            faces,
            resolution,
        }
    }

    pub fn standard_gravitational_parameter(&self) -> f32 {
        G * self.mass
    }

    pub fn calculate_velocity_at_radius(&self, radius: f32) -> f32 {
        (self.standard_gravitational_parameter() / radius).sqrt()
    }

    pub fn escape_velocity(&self) -> f32 {
        let n = 2.0 * self.standard_gravitational_parameter();
        let d = self.settings.radius;
        let nd = n / d;

        nd.sqrt()
    }

    pub fn update(&mut self, _dt: f32) {
        //let rotation_speed_deg: f32 = 0.01;
        //let rotation_speed: f32 = rotation_speed_deg * f32::PI() / 180.0;

        //let rot: Quaternion<f32> = Quaternion::from_axis_angle(
        //    Vector3::new(1.0, 0.0, 0.0).normalize(),
        //    Rad(rotation_speed),
        //);
        //self.rotation = self.rotation * rot;

        //let force = self.mass * self.velocity

        //let new_pos:  Vector3<f32> = Vector3::new(0.0, 0.01, 0.0);
        //self.position = self.position + new_pos;

        self.transform.position += self.velocity; //* dt.as_secs_f32();
                                                  //self.position = self.position + (self.velocity * _dt.as_secs_f32() * SIM_SPEED);

        // Update the uniform buffer
        self.uniform_buffer.data.model = self.transform.calculate_model_matrix();
        self.uniform_buffer.data.normal = self.transform.calculate_normal_matrix();
    }

    pub fn rebuild_mesh(&mut self, renderer: &vesta::Renderer) {
        for face in self.faces.iter_mut() {
            face.construct_mesh(renderer, self.settings, &self.generator);
        }
    }
}

/// Settings that define how a celestial body will look like and react
#[derive(Copy, Clone, Debug)]
pub struct CelestialBodySettings {
    pub radius: f32,
    pub terrain: CelestialBodyTerrain,
}

#[derive(Copy, Clone, Debug)]
pub struct CelestialBodyTerrain {
    pub strength: f32,
    pub num_layers: usize,
    pub base_roughness: f32,
    pub roughness: f32,
    pub persistence: f32,
    pub center: Vector3<f32>,
    pub min_value: f32,
}

impl Default for CelestialBodyTerrain {
    fn default() -> Self {
        Self {
            strength: 1.0,
            num_layers: 1,
            base_roughness: 1.0,
            roughness: 2.0,
            persistence: 0.5,
            center: (0.0, 0.0, 0.0).into(),
            min_value: 0.0,
        }
    }
}

pub struct CelestialBodyTerrainGenerator {
    noise: FastNoise,
}

impl CelestialBodyTerrainGenerator {
    pub fn new() -> Self {
        Self {
            noise: FastNoise::new(),
        }
    }

    pub fn evaluate(&self, point: Vector3<f32>, settings: CelestialBodySettings) -> f32 {
        let ts = settings.terrain;

        let mut noise_val = 0.0;
        let mut frequency = ts.base_roughness;
        let mut amplitude = 1.0;

        for _i in 0..ts.num_layers {
            let v = self.noise.get_noise3d(
                point.x * frequency + ts.center.x,
                point.y * frequency + ts.center.y,
                point.z * frequency + ts.center.z,
            );

            noise_val += (v + 1.0) * 0.5 * amplitude;

            frequency *= ts.roughness;
            amplitude *= ts.persistence;
        }

        noise_val = Math::max(0.0, noise_val - ts.min_value);
        noise_val * ts.strength
    }
}
