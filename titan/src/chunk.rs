use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier3d::prelude::*;

use crate::{
    block_map::{
        add_uvs, texture_offset_from_block, vertex_offset, FACE_BACK, FACE_BOTTOM, FACE_FRONT,
        FACE_LEFT, FACE_RIGHT, FACE_TOP, INDEX_MAP, NORMAL_MAP, TEXTURE_MAP, VERTEX_MAP,
    },
    terrain::Terrain,
};

// Chunk constants

pub const CHUNK_XZ: usize = 32;
pub const CHUNK_Y: usize = 16;
pub const CHUNK_SZ: usize = CHUNK_XZ * CHUNK_XZ * CHUNK_Y;

pub const WORLD_XZ: isize = 16;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum VoxelType {
    #[default]
    Air,
    Dirt,
    Grass,
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
        blocks.resize(CHUNK_SZ, VoxelType::Air);
        Self { blocks }
    }
}

impl Chunk {
    /// Set the block type at the provided position
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, voxel_type: VoxelType) {
        self.blocks[(z * CHUNK_XZ * CHUNK_Y + y * CHUNK_XZ + x) as usize] = voxel_type;
    }

    /// Get the block type at the provided position
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
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut index = 0;

        for x in 0..CHUNK_XZ {
            for y in 0..CHUNK_Y {
                for z in 0..CHUNK_XZ {
                    let xf = x as f32;
                    let yf = y as f32;
                    let zf = z as f32;

                    let block_type = self.get_block(x, y, z);
                    if block_type == VoxelType::Air {
                        continue;
                    }

                    let texture_offset = texture_offset_from_block(block_type);

                    // Top Face
                    if self.is_transparent(xf, yf + 1.0, zf) {
                        for i in 0..4 {
                            vertices.push(vertex_offset(VERTEX_MAP[FACE_TOP][i], xf, yf, zf));
                            normals.push(NORMAL_MAP[FACE_TOP][i]);
                            uvs.push(add_uvs(texture_offset.top, TEXTURE_MAP[FACE_TOP][i]));
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
                            uvs.push(add_uvs(texture_offset.bottom, TEXTURE_MAP[FACE_BOTTOM][i]));
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
                            uvs.push(add_uvs(texture_offset.right, TEXTURE_MAP[FACE_RIGHT][i]));
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
                            uvs.push(add_uvs(texture_offset.left, TEXTURE_MAP[FACE_LEFT][i]));
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
                            uvs.push(add_uvs(texture_offset.front, TEXTURE_MAP[FACE_FRONT][i]));
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
                            uvs.push(add_uvs(texture_offset.back, TEXTURE_MAP[FACE_BACK][i]));
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
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));

        return mesh;
    }
}

pub fn chunk_setup(
    mut commands: Commands,
    terrain: Res<Terrain>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let block_map_texture_handle = asset_server.load("block_map.png");
    let chunk_mat = materials.add(StandardMaterial {
        base_color_texture: Some(block_map_texture_handle.clone()),
        ..Default::default()
    });

    for x in -WORLD_XZ..WORLD_XZ {
        for z in -WORLD_XZ..WORLD_XZ {
            // Where this chunk is in the world
            let world_position = Vec3::new(
                (x * CHUNK_XZ as isize) as f32,
                0.0,
                (z * CHUNK_XZ as isize) as f32,
            );

            // Create a default chunk
            let mut chunk = Chunk::default();

            // Load in some initial terrain
            for cx in 0..CHUNK_XZ {
                for cy in 0..CHUNK_Y {
                    for cz in 0..CHUNK_XZ {
                        let c_pos = Vec3::new(cx as f32, cy as f32, cz as f32) + world_position;
                        let block_type = terrain.get_block_type(c_pos);

                        chunk.set_block(cx, cy, cz, block_type);
                    }
                }
            }

            let chunk_mesh_handle = meshes.add(chunk.create_mesh());
            let chunk_mesh = &meshes.get(&chunk_mesh_handle);

            commands
                .spawn_bundle(ChunkBundle {
                    chunk,
                    material: chunk_mat.clone(),
                    transform: Transform::from_translation(world_position),
                    ..Default::default()
                })
                .insert(chunk_mesh_handle)
                .insert(RigidBody::Fixed)
                .insert(
                    Collider::from_bevy_mesh(chunk_mesh.unwrap(), &ComputedColliderShape::TriMesh)
                        .unwrap(),
                );
        }
    }
}
