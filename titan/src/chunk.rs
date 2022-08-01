use bevy::{
    math::Vec3,
    prelude::{Component, Mesh},
};

// Chunk constants

pub const CHUNK_XZ: usize = 16;
pub const CHUNK_Y: usize = 32;
pub const CHUNK_SZ: usize = CHUNK_XZ * CHUNK_Y;

#[derive(Clone, Copy, PartialEq)]
pub enum VoxelType {
    Air,
    Dirt,
}

/// Represents a single chunk in the world
#[derive(Component)]
pub struct Chunk {
    /// The mesh for this chunk
    pub mesh: Option<Mesh>,

    /// The position this chunk is in
    pub position: Vec3,

    /// 1D Array of all blocks in this chunk
    pub blocks: Vec<VoxelType>,
}

impl Chunk {
    pub fn new(position: Vec3) -> Self {
        let mut blocks = Vec::with_capacity(CHUNK_SZ);
        blocks.resize(CHUNK_SZ, VoxelType::Dirt);
        Self {
            mesh: None,
            position,
            blocks,
        }
    }
}
