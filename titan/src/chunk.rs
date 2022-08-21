use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier3d::prelude::*;

use crate::block_map::{
    vertex_offset, FACE_BACK, FACE_BOTTOM, FACE_FRONT, FACE_LEFT, FACE_RIGHT, FACE_TOP, INDEX_MAP,
    NORMAL_MAP, VERTEX_MAP,
};

// Chunk constants

pub const CHUNK_XZ: usize = 16;
pub const CHUNK_Y: usize = 16;
pub const CHUNK_SZ: usize = CHUNK_XZ * CHUNK_XZ * CHUNK_Y;

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

        blocks[(0 * CHUNK_XZ * CHUNK_Y + 15 * CHUNK_XZ + 0)] = VoxelType::Air;
        Self { blocks }
    }
}

impl Chunk {
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> VoxelType {
        self.blocks[(z * CHUNK_XZ * CHUNK_Y + y * CHUNK_XZ + x) as usize]
    }

    /// Returns if the specified block is transparent (air, water, etc.)
    /// Used for block culling
    fn is_transparent(&self, x: f32, y: f32, z: f32) -> bool {
        // Always air on top of a chunk
        if y >= CHUNK_Y as f32 {
            return true;
        }

        // No need to render the bottom of the world
        if y < 0.0 {
            return false;
        }

        // If outside this chunk
        if (x < 0.0) || (z < 0.0) || (x >= CHUNK_XZ as f32) || (z >= CHUNK_XZ as f32) {
            // Always true for now
            return true;
        }

        return self.get_block(x as usize, y as usize, z as usize) == VoxelType::Air;
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

                    // Top Face
                    if self.is_transparent(xf, yf + 1.0, zf) {
                        for i in 0..4 {
                            vertices.push(vertex_offset(VERTEX_MAP[FACE_TOP][i], xf, yf, zf));
                            normals.push(NORMAL_MAP[FACE_TOP][i]);
                        }

                        for i in 0..6 {
                            indices.push(index + INDEX_MAP[FACE_TOP][i])
                        }

                        index += 4;
                    }

                    // Bottom Face
                    if self.is_transparent(xf, yf - 1.0, zf) {
                        for i in 0..4 {
                            vertices.push(vertex_offset(VERTEX_MAP[FACE_BOTTOM][i], xf, yf, zf));
                            normals.push(NORMAL_MAP[FACE_BOTTOM][i]);
                        }

                        for i in 0..6 {
                            indices.push(index + INDEX_MAP[FACE_BOTTOM][i])
                        }

                        index += 4;
                    }

                    // Right Face
                    if self.is_transparent(xf + 1.0, yf, zf) {
                        for i in 0..4 {
                            vertices.push(vertex_offset(VERTEX_MAP[FACE_RIGHT][i], xf, yf, zf));
                            normals.push(NORMAL_MAP[FACE_RIGHT][i]);
                        }

                        for i in 0..6 {
                            indices.push(index + INDEX_MAP[FACE_RIGHT][i])
                        }

                        index += 4;
                    }

                    // Left Face
                    if self.is_transparent(xf - 1.0, yf, zf) {
                        for i in 0..4 {
                            vertices.push(vertex_offset(VERTEX_MAP[FACE_LEFT][i], xf, yf, zf));
                            normals.push(NORMAL_MAP[FACE_LEFT][i]);
                        }

                        for i in 0..6 {
                            indices.push(index + INDEX_MAP[FACE_LEFT][i])
                        }

                        index += 4;
                    }

                    // Front Face
                    if self.is_transparent(xf, yf, zf + 1.0) {
                        for i in 0..4 {
                            vertices.push(vertex_offset(VERTEX_MAP[FACE_FRONT][i], xf, yf, zf));
                            normals.push(NORMAL_MAP[FACE_FRONT][i]);
                        }

                        for i in 0..6 {
                            indices.push(index + INDEX_MAP[FACE_FRONT][i])
                        }

                        index += 4;
                    }

                    // Back Face
                    if self.is_transparent(xf, yf, zf - 1.0) {
                        for i in 0..4 {
                            vertices.push(vertex_offset(VERTEX_MAP[FACE_BACK][i], xf, yf, zf));
                            normals.push(NORMAL_MAP[FACE_BACK][i]);
                        }

                        for i in 0..6 {
                            indices.push(index + INDEX_MAP[FACE_BACK][i])
                        }

                        index += 4;
                    }
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
