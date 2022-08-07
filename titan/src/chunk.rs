use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier3d::prelude::*;

use crate::block_map::{vertex_offset, FACE_FRONT, INDEX_MAP, NORMAL_MAP, VERTEX_MAP};

// Chunk constants

pub const CHUNK_XZ: usize = 16;
pub const CHUNK_Y: usize = 16;
pub const CHUNK_SZ: usize = CHUNK_XZ * CHUNK_Y;

pub const WORLD_XZ: isize = 4;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum VoxelType {
    #[default]
    Air,
    Dirt,
}

/// Represents a single chunk in the world
#[derive(Component)]
pub struct Chunk {
    /// 1D Array of all blocks in this chunk
    pub blocks: Vec<VoxelType>,
}

#[derive(Default, Bundle)]
pub struct ChunkBundle {
    /// Chunk data
    pub chunk: Chunk,
    /// The chunk material (this is standard)
    pub material: Handle<StandardMaterial>,
    /// Where the chunk is located in the world
    pub transform: Transform,
    /// Global world transform
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for Chunk {
    fn default() -> Self {
        let mut blocks = Vec::with_capacity(CHUNK_SZ);
        blocks.resize(CHUNK_SZ, VoxelType::Dirt);
        Self { blocks }
    }
}

impl Chunk {
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> VoxelType {
        self.blocks[(z * CHUNK_XZ * CHUNK_Y + y * CHUNK_XZ + x) as usize]
    }

    pub fn create_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();

        let mut index = 0;

        for x in 0..CHUNK_XZ {
            for y in 0..CHUNK_Y {
                for z in 0..CHUNK_XZ {
                    let xf = x as f32;
                    let yf = y as f32;
                    let zf = z as f32;

                    for i in 0..4 {
                        vertices.push(vertex_offset(VERTEX_MAP[FACE_FRONT][i], xf, yf, zf));
                        normals.push(NORMAL_MAP[FACE_FRONT][i]);
                    }

                    for i in 0..6 {
                        indices.push(index + INDEX_MAP[FACE_FRONT][i])
                    }

                    index += 4;
                }
            }
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));

        return mesh;
    }
}

pub fn chunk_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk_mat = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    for x in -WORLD_XZ..WORLD_XZ {
        for z in -WORLD_XZ..WORLD_XZ {
            let chunk = Chunk::default();
            let chunk_mesh = meshes.add(chunk.create_mesh());

            meshes.add(Mesh::from(shape::Cube {
                size: CHUNK_XZ as f32,
            }));

            commands
                .spawn_bundle(ChunkBundle {
                    chunk,
                    material: chunk_mat.clone(),
                    transform: Transform::from_xyz(
                        (x * CHUNK_XZ as isize) as f32,
                        0.0,
                        (z * CHUNK_XZ as isize) as f32,
                    ),
                    ..Default::default()
                })
                .insert(chunk_mesh)
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(
                    CHUNK_XZ as f32 / 2.0,
                    CHUNK_XZ as f32 / 2.0,
                    CHUNK_XZ as f32 / 2.0,
                ));
        }
    }
}
