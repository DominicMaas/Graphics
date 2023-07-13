use bevy::prelude::*;
use bracket_noise::prelude::*;

use crate::chunk::{Chunk, VoxelType, CHUNK_XZ, CHUNK_Y, WORLD_HEIGHT};

#[derive(Resource)]
pub struct Terrain {
    pub noise_func: FastNoise,
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

        Self { noise_func }
    }

    pub fn generate2(&self, world_position: Vec3) -> Chunk {
        let mut chunk = Chunk::new();

        // Load in some initial terrain
        for cx in 0..CHUNK_XZ {
            for cy in 0..CHUNK_Y {
                for cz in 0..CHUNK_XZ {
                    let c_pos = Vec3::new(cx as f32, cy as f32, cz as f32) + world_position;
                    let block_type = self.get_block_type(c_pos);

                    chunk.set_block(cx, cy, cz, block_type);
                }
            }
        }

        chunk
    }

    pub fn generate(&self, chunk: &mut Chunk, world_position: Vec3) {
        // Load in some initial terrain
        for cx in 0..CHUNK_XZ {
            for cy in 0..CHUNK_Y {
                for cz in 0..CHUNK_XZ {
                    let c_pos = Vec3::new(cx as f32, cy as f32, cz as f32) + world_position;
                    let block_type = self.get_block_type(c_pos);

                    chunk.set_block(cx, cy, cz, block_type);
                }
            }
        }
    }

    /// Gets the block type at this position
    pub fn get_block_type(&self, position: Vec3) -> VoxelType {
        let raw_noise =
            self.noise_func
                .get_noise3d(position.x / 255.0, position.y / 255.0, position.z / 255.0);

        assert!(raw_noise >= -1.0);
        assert!(raw_noise <= 1.0);

        // Map this noise between 0 and world height
        let terrain_noise = Self::map_range((-1.0, 1.0), (0.0, WORLD_HEIGHT as f32), raw_noise);

        assert!(terrain_noise >= 0.0);
        assert!(terrain_noise <= WORLD_HEIGHT as f32);

        //println!("TN: {}", terrain_noise);

        let mut t = VoxelType::Air;

        // Calculate the density of the terrain at this point, 0.0 is air,
        // 1.0 is full underground, between these values is a range
        let density: f32;
        if terrain_noise > position.y - 1.0 {
            // 55.5 > 55-1
            // Fully underground
            density = 1.0;
        } else if terrain_noise < position.y + 1.0 {
            // 55.5 3042 < 55+1
            // Fully Aboveground
            density = 0.0;
        } else {
            // Partial
            density = f32::abs(terrain_noise - position.y);
            println!("DN: {}", density);
        }

        // If the generated terrain noise is above our current height (or equal),
        // set the block to sand. This effectivity paints the world with voxels alongside
        // the noise
        if terrain_noise >= position.y {
            assert!(density > 0.0);
            assert!(density <= 1.0);

            t = VoxelType::Stone;
        }

        if terrain_noise >= (position.y - 1.0) {
            t = VoxelType::Grass;
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

    fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
        to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
    }
}
