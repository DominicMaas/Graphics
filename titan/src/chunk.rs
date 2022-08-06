use bevy::prelude::*;

// Chunk constants

pub const CHUNK_XZ: usize = 16;
pub const CHUNK_Y: usize = 32;
pub const CHUNK_SZ: usize = CHUNK_XZ * CHUNK_Y;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum VoxelType {
    #[default]
    Air,
    Dirt,
}

/// Represents a single chunk in the world
#[derive(Default, Component)]
pub struct Chunk {
    /// 1D Array of all blocks in this chunk
    pub blocks: Vec<VoxelType>,
}

#[derive(Default, Bundle)]
struct ChunkBundle {
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

impl Chunk {
    pub fn new() -> Self {
        let mut blocks = Vec::with_capacity(CHUNK_SZ);
        blocks.resize(CHUNK_SZ, VoxelType::Dirt);
        Self { blocks }
    }
}
