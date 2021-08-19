pub mod block_map;
pub mod chunk;
pub mod generator;
pub mod world;

pub use block_map::*;
pub use chunk::*;
pub use generator::*;
pub use world::*;

//pub type BlockType = u16;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Air,
    Dirt,
    Sand,
    Grass,
    Stone,
    Water,
}

pub const CHUNK_HEIGHT: u32 = 64;
pub const CHUNK_WIDTH: u32 = 16;
