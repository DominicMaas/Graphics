use bevy::prelude::*;
use bracket_noise::prelude::*;

use crate::chunk::{TerrainVoxel, VoxelType};

pub struct Terrain {
    seed: u64,
    noise_func: FastNoise,
    water_level: f32,
}

impl Terrain {
    pub fn new(seed: u64) -> Self {
        let mut noise_func = FastNoise::seeded(seed);
        noise_func.set_noise_type(NoiseType::SimplexFractal);
        noise_func.set_fractal_type(FractalType::FBM);
        noise_func.set_fractal_octaves(6);
        noise_func.set_fractal_gain(0.5);
        noise_func.set_fractal_lacunarity(2.0);
        noise_func.set_frequency(0.6);

        Self {
            seed,
            noise_func,
            water_level: 6.0,
        }
    }

    /// Gets the block type at this position
    pub fn get_block_type(&self, position: Vec3) -> VoxelType {
        // Val between -1 and 1
        let noise =
            self.noise_func
                .get_noise3d(position.x / 140.0, position.y / 160.0, position.z / 140.0);

        let mut y_off = noise * 32.0;
        y_off += 12.0;

        let mut t = VoxelType::Air;

        if y_off >= position.y {
            t = VoxelType::Dirt(TerrainVoxel { density: noise });
        }

        // Get top layer grass
        //if t == VoxelType::Dirt(_) {
        //    if self.get_block_type(position + Vec3::new(0.0, 1.0, 0.0)) == VoxelType::Air {
        //        t = VoxelType::Grass;
        //    }
        // }

        t

        // Build noise
        /*let noise = self.noise_func.get_noise3d(
            position.x * 2. + 5.0,
            position.y * 2. + 3.0,
            position.z * 2. + 0.6,
        );

        //let noise = self
        //    .noise_func
        //    .get_noise(position.x * 2. + 5.0, position.z * 2. + 0.6);

        let v = position.y + noise;

        //  ahh
        let sn = 1f32
            - (position.x * position.x + position.y * position.y + position.z * position.z).sqrt()
                / 5f32;

        // println!("POS:  {}, V: {}, N: {}", position.y, v, sn);

        if v > 0.0 {
            VoxelType::Dirt(TerrainVoxel { density: noise })
        } else {
            VoxelType::Air
        }

        /*

          let up = Vec3::new(0.0, 1.0, 0.0);

         v *= 32.0;

         v += 12.0;

        let mut t = VoxelType::Air;

         if v >= position.y {
             t = VoxelType::Dirt(TerrainVoxel { density: v })
         }

         match t {
             // Get top layer grass
             VoxelType::Dirt(_) => {
                 if self.get_block_type(position + up) == VoxelType::Air {
                     t = VoxelType::Grass(TerrainVoxel { density: v });
                 }
             }
             // Replace air below water level with water
             VoxelType::Air => {
                 if position.y <= self.water_level {
                     t = VoxelType::Water;
                 }
             }
             _ => (),
         }

         // Bottom of the world should be dirt
         if position.y == 0.0 {
             t = VoxelType::Dirt(TerrainVoxel { density: 0.0 });
         }

         t*/*/
    }
}
