#![allow(dead_code)]

pub mod material;
pub mod mesher;
pub mod tile_map;

use crate::{
    table::{VoxelFace, FACE_BOTTOM, FACE_TOP},
    terrain::Terrain,
};
use bevy::{prelude::*, render::texture::ImageSampler};
use bevy_rapier3d::prelude::*;
use bevy_tile_atlas::TileAtlasBuilder;
use std::collections::VecDeque;

use self::{material::ChunkMaterial, mesher::ChunkMesher, tile_map::TileAssets};

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId {
    pub pos: IVec2,
}

impl ChunkId {
    pub fn new(x: isize, z: isize) -> Self {
        Self {
            pos: IVec2::new(
                (x * CHUNK_XZ as isize) as i32,
                (z * CHUNK_XZ as isize) as i32,
            ),
        }
    }
}

// Chunk constants

pub const CHUNK_XZ: usize = 16;
pub const CHUNK_Y: usize = 64;
pub const CHUNK_SZ: usize = CHUNK_XZ * CHUNK_XZ * CHUNK_Y;

pub const WORLD_XZ: isize = 18;
pub const WORLD_Y: isize = 1;

pub const WORLD_HEIGHT: usize = WORLD_Y as usize * CHUNK_Y;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum VoxelType {
    #[default]
    Air,
    Dirt,
    Grass,
    Leaf,
    Log,
    Stone,
    Sand,
    Glass,
    Brick,
    Water,
}

impl VoxelType {
    /// Get the texture index of rhis voxel type
    pub fn texture_index(&self, face: VoxelFace) -> u32 {
        match self {
            VoxelType::Dirt => 0,
            VoxelType::Grass => match face {
                FACE_TOP => 1,
                FACE_BOTTOM => 0,
                _ => 2,
            },
            VoxelType::Stone => 3,
            _ => 0,
        }
    }

    /// Get the numerical index of this voxel type
    pub fn index(&self) -> u32 {
        *self as u32
    }
}

/// Represents the state of a chunk, this is useful for
/// keeping track of the chunk throughout the world
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum ChunkState {
    /// The chunk has been created but there is no information associated with it
    /// aka it is empty. This chunk does not have an associated bundle in the scene
    #[default]
    Empty,

    /// The chunk is currently generating its voxel information. It should not be touched
    /// until this is complete. Voxel generation happens in separate threads
    Generating,

    /// The chunk has voxel information associated with it, but no mesh has been
    /// generated, nor bundle added to the scene
    Generated,

    /// The chunk has a mesh associated with it, and has voxel information. It's currently
    /// in the scene.
    Loaded,

    /// The chunk is currently dirty and needs to be rebuilt, this involves replacing the mesh
    /// that is in the scene with a new mesh. This is done when the chunk is modified.
    Dirty,
}

/// Represents a single chunk in the world
#[derive(Component, Debug)]
pub struct Chunk {
    /// What state the chunks is in, this determines how this chunk is treated in the world
    pub state: ChunkState,

    /// 1D Array of all blocks in this chunk
    pub blocks: Vec<VoxelType>,
}

#[derive(Default, Bundle)]
pub struct ChunkBundle {
    /// The id of this chunk, used to link up to the world
    pub chunk_id: ChunkId,
    /// The chunk material (this is standard)
    pub material: Handle<ChunkMaterial>,
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
        Self {
            state: ChunkState::Empty,
            blocks,
        }
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

    /// Get the block type at the provided position
    fn get_t_block(
        &self,
        world_position: Vec3,
        position: Vec3,
        terrain: &Res<Terrain>,
    ) -> VoxelType {
        // If outside this chunk
        if (position.x < 0.0)
            || (position.y < 0.0)
            || (position.z < 0.0)
            || (position.x >= CHUNK_XZ as f32)
            || (position.y >= CHUNK_Y as f32)
            || (position.z >= CHUNK_XZ as f32)
        {
            return terrain.get_block_type(world_position + position);
        }

        // If inside the chunk
        self.blocks[(position.z as usize * CHUNK_XZ * CHUNK_Y
            + position.y as usize * CHUNK_XZ
            + position.x as usize) as usize]
    }

    /// Returns if the specified block is transparent (air, water, etc.)
    /// Used for block culling
    fn is_transparent(&self, world_position: Vec3, position: Vec3, terrain: &Res<Terrain>) -> bool {
        self.get_t_block(world_position, position, terrain) == VoxelType::Air
    }
}

pub fn chunk_setup(
    mut commands: Commands,
    terrain: Res<Terrain>,
    mut world: ResMut<crate::world::World>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut textures: ResMut<Assets<Image>>,
    tile_assets: Res<TileAssets>,
) {
    let mut builder = TileAtlasBuilder::new(Vec2::new(16.0, 16.0));

    // Add our textures
    for handle in tile_assets.tiles.iter() {
        let texture = textures.get(handle).unwrap();
        builder.add_texture(handle.clone(), texture).unwrap();
    }

    // Vertically stacked
    builder.max_columns(Some(1));

    // Build our atlas
    let atlas = builder.finish(&mut textures).unwrap();

    // Reinterpret our image as a stacked 2d array, and use near sampling
    // (our textures are pixel art)
    if let Some(atlas_image) = textures.get_mut(&atlas.texture) {
        atlas_image.reinterpret_stacked_2d_as_array(atlas.len() as u32);
        atlas_image.sampler_descriptor = ImageSampler::nearest();
    }

    let chunk_mat = materials.add(ChunkMaterial {
        texture: atlas.texture,
    });

    for x in -WORLD_XZ..WORLD_XZ {
        for z in -WORLD_XZ..WORLD_XZ {
            // Where this chunk is in the world
            let world_position = Vec3::new(
                (x * CHUNK_XZ as isize) as f32,
                0.0,
                (z * CHUNK_XZ as isize) as f32,
            );

            // Generate the unique id for our chunk
            let id = ChunkId::new(x, z);

            // Insert our chunk into the world
            world.chunks.insert(id, Chunk::default());

            // Now get this chunk back as we are going to
            // immediately start doing stuff to it
            if let Some(chunk) = world.chunks.get_mut(&id) {
                // Generate the chunk
                terrain.generate(chunk, world_position);

                // Create a mesh for our chunk
                if let Some(m) = ChunkMesher::build(chunk, world_position, &terrain) {
                    let chunk_mesh_handle = meshes.add(m);
                    let chunk_mesh = &meshes.get(&chunk_mesh_handle);

                    commands
                        .spawn(ChunkBundle {
                            chunk_id: id,
                            material: chunk_mat.clone(),
                            transform: Transform::from_translation(world_position),
                            ..Default::default()
                        })
                        .insert(chunk_mesh_handle)
                        .insert(RigidBody::Fixed)
                        .insert(Name::new(format!("Chunk: {}", world_position)))
                        .insert(
                            Collider::from_bevy_mesh(
                                chunk_mesh.unwrap(),
                                &ComputedColliderShape::TriMesh,
                            )
                            .unwrap(),
                        );

                    // In the world!
                    chunk.state = ChunkState::Loaded;
                }
            }
        }
    }
}
