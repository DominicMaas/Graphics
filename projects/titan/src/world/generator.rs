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
        let frequency = 1.0;
        let noise_height = 0.0;

        let mut v = self
            .noise_func
            .get([position.x as f64, position.y as f64, position.z as f64]);

        v += 10.0;

        if v > position.y as f64 {
            return 1;
        } else {
            return 0;
        }
    }
}
