use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use bracket_noise::prelude::*;
use bracket_random::prelude::*;

use crate::GAME_SCALE;

pub const CHUNK_X: usize = 32;
pub const CHUNK_Y: usize = 32;
pub const CHUNK_SZ: usize = CHUNK_X * CHUNK_Y;

pub struct SpawnChunkEvent(pub Vec2);

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    #[default]
    Air,
    Grass,
    Dirt,
    RockEntity,
}

impl TileType {
    pub fn is_opaque(&self) -> bool {
        match self {
            TileType::Air => false,
            TileType::RockEntity => false,
            _ => true,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct ChunkResources {
    pub material: Handle<ColorMaterial>,
    pub tile_size: Vec2,
    pub rows: usize,
    pub columns: usize,
    pub seed: u64,
}

impl ChunkResources {
    pub fn get_uv_coords(&self, position: Vec2) -> [[f32; 2]; 4] {
        let width = self.columns as f32 * self.tile_size.x;
        let height = self.rows as f32 * self.tile_size.y;

        let uv_l = (position.x * self.tile_size.x) / width; //0
        let uv_r = ((position.x + 1.0) * self.tile_size.x) / width; //1
        let uv_t = (position.y * self.tile_size.y) / height; //0
        let uv_b = ((position.y + 1.0) * self.tile_size.y) / height; //1

        let bleed_padding = 0.001;

        [
            [uv_l + bleed_padding, uv_b - bleed_padding],
            [uv_l + bleed_padding, uv_t + bleed_padding],
            [uv_r - bleed_padding, uv_t + bleed_padding],
            [uv_r - bleed_padding, uv_b - bleed_padding],
        ]
    }
}

#[derive(Component, Clone)]
pub struct Chunk {
    /// Position of this chunk in the world, used for mesh generation
    pub position: Vec2,
    /// 1D Array of all tiles in this chunk
    pub tiles: Vec<TileType>,
}

#[derive(Bundle, Clone)]
pub struct ChunkBundle {
    /// Parent bundle to help keep things clean!
    #[bundle]
    pub material: MaterialMesh2dBundle<ColorMaterial>,

    /// Chunk data
    pub chunk: Chunk,
}

impl Chunk {
    fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
        to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
    }

    pub fn new(position: Vec2, seed: u64) -> Self {
        let mut chunk = Chunk {
            tiles: Vec::with_capacity(CHUNK_SZ),
            position,
        };

        // Default to Air tiles (ensure the vector is the right size)
        chunk.tiles.resize(CHUNK_SZ, TileType::Air);

        // Setup our terrain generation (todo: move this into a seperate module)
        //   NoiseBuilder::

        //   let noise2 = NoiseBuilder::fbm_1d(100, 100).generate_scaled(0.0, 1.0);

        let mut rng = RandomNumberGenerator::new();
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(6);
        noise.set_frequency(0.6);

        for x in 0..CHUNK_X {
            // Generate the noise, map it, and offset it
            let mut n = noise.get_noise((chunk.position.x + (x as f32)) / 48.0, 0.0);
            n = Self::map_range((-1.0, 1.0), (0.0, CHUNK_Y as f32 - 5.0), n);
            n += 5.0;

            // Ensure we have a floor for the world
            chunk.set_tile(x, 0, TileType::Dirt);

            // Now generate the rest
            for y in 1..n as usize {
                chunk.set_tile(x, y, TileType::Dirt);
            }

            // Ensure we have grass on top
            chunk.set_tile(x, n as usize, TileType::Grass);

            // There is a random chance to have a rock on top of this tile!
            // if n as usize + 1 <= CHUNK_Y && rng.range(0, 100) < 20 {
            //      chunk.set_tile(x, n as usize + 1, TileType::RockEntity);
            // }
        }

        chunk
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[CHUNK_X * x + y] = tile;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        if x >= CHUNK_X {
            return TileType::Air;
        }

        if y >= CHUNK_Y {
            return TileType::Air;
        }

        self.tiles[CHUNK_X * x + y]
    }

    pub fn create_mesh(&self, chunk_resources: &Res<ChunkResources>) -> Mesh {
        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut m = 0;

        let mut rng = RandomNumberGenerator::new();

        for x in 0..(CHUNK_X) {
            for y in 0..(CHUNK_Y) {
                let tile = self.get_tile(x, y);

                let tile_left = if x != 0 {
                    self.get_tile(x - 1, y)
                } else {
                    TileType::Air
                };

                let tile_right = self.get_tile(x + 1, y);

                let tile_top_right = self.get_tile(x + 1, y + 1);

                // Don't build air
                if tile == TileType::Air {
                    continue;
                }

                let xf = x as f32;
                let yf = y as f32;

                let uvs = chunk_resources.get_uv_coords(match tile {
                    TileType::Air => Vec2::new(0.0, 0.0),
                    TileType::Grass => match (tile_left.is_opaque(), tile_right.is_opaque()) {
                        (false, false) => Vec2::new(3.0, 1.0),
                        (false, _) => Vec2::new(1.0, 1.0),
                        (_, false) => Vec2::new(0.0, 1.0),
                        _ => match rng.range(0, 4) {
                            0 => Vec2::new(0.0, 0.0),
                            1 => Vec2::new(1.0, 0.0),
                            3 => Vec2::new(2.0, 0.0),
                            _ => Vec2::new(3.0, 0.0),
                        },
                    },
                    TileType::Dirt => match rng.range(0, 4) {
                        0 => Vec2::new(0.0, 7.0),
                        1 => Vec2::new(2.0, 7.0),
                        2 => Vec2::new(2.0, 7.0),
                        _ => Vec2::new(3.0, 7.0),
                    },
                    TileType::RockEntity => match rng.range(0, 3) {
                        0 => Vec2::new(16.0, 6.0),
                        1 => Vec2::new(16.0, 5.0),
                        _ => Vec2::new(14.0, 4.0),
                    },
                });

                vertices.push(([xf + -0.5, yf + -0.5, 0.0], [0.0, 0.0, 1.0], uvs[0]));
                vertices.push(([xf + -0.5, yf + 0.5, 0.0], [0.0, 0.0, 1.0], uvs[1]));
                vertices.push(([xf + 0.5, yf + 0.5, 0.0], [0.0, 0.0, 1.0], uvs[2]));
                vertices.push(([xf + 0.5, yf + -0.5, 0.0], [0.0, 0.0, 1.0], uvs[3]));

                indices.push(m + 0);
                indices.push(m + 2);
                indices.push(m + 1);
                indices.push(m + 0);
                indices.push(m + 3);
                indices.push(m + 2);

                m += 4;
            }
        }

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }

    pub fn create_colider_mesh(&self) -> Collider {
        let mut vertices: Vec<Vect> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut m = 0;

        for x in 0..(CHUNK_X) {
            for y in 0..(CHUNK_Y) {
                // Don't build air
                if !self.get_tile(x, y).is_opaque() {
                    continue;
                }

                let xf = x as f32;
                let yf = y as f32;

                vertices.push([xf + -0.5, yf + -0.5].into());
                vertices.push([xf + -0.5, yf + 0.5].into());
                vertices.push([xf + 0.5, yf + 0.5].into());
                vertices.push([xf + 0.5, yf + -0.5].into());

                indices.push(m + 0);
                indices.push(m + 2);
                indices.push(m + 1);
                indices.push(m + 0);
                indices.push(m + 3);
                indices.push(m + 2);

                m += 4;
            }
        }

        Collider::polyline(vertices, None)
    }
}

pub fn spawn_chunk_system(
    mut ev_spawn_chunk: EventReader<SpawnChunkEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    chunk_resources: Res<ChunkResources>,
) {
    for ev in ev_spawn_chunk.iter() {
        let chunk = Chunk::new(ev.0, chunk_resources.seed);
        let mesh = chunk.create_mesh(&chunk_resources);

        commands.spawn((
            ChunkBundle {
                chunk: chunk.clone(),
                material: MaterialMesh2dBundle {
                    mesh: meshes.add(mesh).into(),
                    transform: Transform::from_xyz(ev.0.x * 8.0, ev.0.y, 0.0)
                        .with_scale(Vec3::splat(8.0)),
                    material: chunk_resources.material.clone(),
                    ..default()
                },
            },
            chunk.create_colider_mesh(),
            RigidBody::Fixed,
        ));
    }
}
