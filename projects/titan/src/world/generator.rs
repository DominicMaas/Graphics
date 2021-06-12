use super::{BlockType, CHUNK_HEIGHT, CHUNK_WIDTH};
use noise::{NoiseFn, OpenSimplex, Seedable};
use vesta::cgmath::{num_traits::clamp, Vector3};

pub struct Generator {
    seed: u32,
    scale: f32,
    num_octaves: usize,
    persistance: f32,
    lacunarity: f32,
    noise_func: OpenSimplex,
}

impl Generator {
    pub fn new(
        seed: u32,
        scale: f32,
        num_octaves: usize,
        persistance: f32,
        lacunarity: f32,
    ) -> Self {
        let noise_func = OpenSimplex::new().set_seed(seed);

        Self {
            seed,
            scale,
            num_octaves,
            persistance,
            lacunarity,
            noise_func,
        }
    }

    pub fn get_theoretical_block_type(&self, position: Vector3<f32>) -> BlockType {
        // Variables needed
        let half_width = (CHUNK_WIDTH / 2) as f32;
        let half_height = (CHUNK_HEIGHT / 2) as f32;

        // Build noise
        let amplitude = 1.0;
        let frequency = 0.05;
        let noise_height = 0.0;

        let mut v = self
            .noise_func
            .get([position.x as f64 * frequency, position.z as f64 * frequency]);

        v = Self::scale(v, -1.0, 1.0, 0.0, (CHUNK_HEIGHT as f64) / 4.0);

        if v >= position.y as f64 {
            return 1;
        } else {
            return 0;
        }
    }

    fn scale(number: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
        return (number - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    }
}
