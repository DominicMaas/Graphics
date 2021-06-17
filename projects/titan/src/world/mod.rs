pub mod block_map;
pub mod chunk;
pub mod generator;
pub mod world;

pub use block_map::*;
pub use chunk::*;
pub use generator::*;
pub use world::*;

pub type BlockType = u16;

pub const CHUNK_HEIGHT: u32 = 128;
pub const CHUNK_WIDTH: u32 = 32;
