use super::BlockType;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use vesta::cgmath::Vector3;

pub struct Generator {
    seed: u64,
    noise_func: FastNoise,
}

impl Generator {
    pub fn new(seed: u64) -> Self {
        let mut noise_func = FastNoise::seeded(seed);
        noise_func.set_noise_type(NoiseType::SimplexFractal);
        noise_func.set_fractal_type(FractalType::FBM);
        noise_func.set_fractal_octaves(6);
        noise_func.set_fractal_gain(0.4);
        noise_func.set_fractal_lacunarity(2.0);
        noise_func.set_frequency(0.008);

        Self { seed, noise_func }
    }

    pub fn get_theoretical_block_type(&self, position: Vector3<f32>) -> BlockType {
        // Build noise
        let mut v = self
            .noise_func
            .get_noise3d(position.x, position.y, position.z);
        v *= 32.0;

        if position.y == 0.0 {
            return BlockType::Water;
        } else if v >= position.y {
            if position.y <= 2.0 {
                return BlockType::Sand;
            } else {
                return BlockType::Grass;
            }
        } else {
            return BlockType::Air;
        }
    }
}
