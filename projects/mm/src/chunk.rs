use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use bracket_noise::prelude::*;
use bracket_random::prelude::*;

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
}

#[derive(Resource, Default, Debug)]
pub struct ChunkResources {
    pub material: Handle<ColorMaterial>,
    pub tile_size: Vec2,
    pub rows: usize,
    pub columns: usize,
}

impl ChunkResources {
    pub fn get_uv_coords(&self, position: Vec2) -> [[f32; 2]; 4] {
        [[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]
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

    pub fn new(position: Vec2) -> Self {
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
        let mut noise = FastNoise::seeded(rng.next_u64());
        noise.set_noise_type(NoiseType::Simplex);
        noise.set_frequency(0.4);

        for x in 0..CHUNK_X {
            // Generate the noise, map it, and offset it
            let mut n = noise.get_noise((chunk.position.x + (x as f32)) / 10.0, 0.0);
            n = Self::map_range((-1.0, 1.0), (0.0, CHUNK_Y as f32 / 2.0), n);
            n += CHUNK_Y as f32 / 2.0;

            // Ensure we have a floor for the world
            chunk.set_tile(x, 0, TileType::Grass);

            // Now generate the rest
            for y in 1..n as usize {
                chunk.set_tile(x, y, TileType::Dirt);
            }

            // Ensure we have grass on top
            chunk.set_tile(x, n as usize, TileType::Grass);
        }

        chunk
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[CHUNK_X * x + y] = tile;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        self.tiles[CHUNK_X * x + y]
    }

    pub fn create_mesh(&self, chunk_resources: Res<ChunkResources>) -> Mesh {
        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut m = 0;

        for x in 0..(CHUNK_X) {
            for y in 0..(CHUNK_Y) {
                // Don't build air
                if self.get_tile(x, y) == TileType::Air {
                    continue;
                }

                let xf = x as f32;
                let yf = y as f32;

                let uvs = chunk_resources.get_uv_coords((0.0, 0.0).into());

                vertices.push(([xf + -0.5, yf + -0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]));
                vertices.push(([xf + -0.5, yf + 0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]));
                vertices.push(([xf + 0.5, yf + 0.5, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]));
                vertices.push(([xf + 0.5, yf + -0.5, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]));

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
                if self.get_tile(x, y) == TileType::Air {
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
        let chunk = Chunk::new(ev.0);
        let mesh = chunk.create_mesh();

        commands.spawn((
            ChunkBundle {
                chunk: chunk.clone(),
                material: MaterialMesh2dBundle {
                    mesh: meshes.add(mesh).into(),
                    transform: Transform::from_xyz(ev.0.x * 25.0, ev.0.y, 0.0)
                        .with_scale(Vec3::splat(25.0)),
                    material: chunk_resources.material.clone(),
                    ..default()
                },
            },
            chunk.create_colider_mesh(),
            RigidBody::Fixed,
        ));
    }
}
