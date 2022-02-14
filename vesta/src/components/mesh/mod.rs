use bevy_ecs::prelude::*;

pub mod cube;
pub mod mesh;

pub use cube::*;
pub use mesh::*;

#[derive(Component)]
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    num_vertices: u32,
}
