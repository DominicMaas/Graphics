use bevy::prelude::*;
use bracket_noise::prelude::*;

use crate::chunk::VoxelType;

pub struct Terrain {
    seed: u64,
    noise_func: FastNoise,
}

impl Terrain {
    pub fn new(seed: u64) -> Self {
        let mut noise_func = FastNoise::seeded(seed);
        noise_func.set_noise_type(NoiseType::SimplexFractal);
        noise_func.set_fractal_type(FractalType::FBM);
        noise_func.set_fractal_octaves(6);
        noise_func.set_fractal_gain(0.5);
        noise_func.set_fractal_lacunarity(2.0);
        noise_func.set_frequency(1.0);

        Self { seed, noise_func }
    }

    /// Gets the block type at this position
    pub fn get_block_type(&self, position: Vec3) -> VoxelType {
        // Build noise
        let mut v =
            self.noise_func
                .get_noise3d(position.x / 140.0, position.y / 100.0, position.z / 140.0);

        v *= 32.0;

        v += 6.0;

        if position.y == 0.0 {
            return VoxelType::Grass;
        }

        if v >= position.y {
            return VoxelType::Dirt;
        } else {
            return VoxelType::Air;
        }
    }
}
