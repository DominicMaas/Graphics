use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::*;

pub const CHUNK_X: usize = 16;
pub const CHUNK_Y: usize = 16;
pub const CHUNK_SZ: usize = CHUNK_X * CHUNK_Y;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum TileType {
    #[default]
    Air,
    Dirt,
}

#[derive(Component, Clone)]
pub struct Chunk {
    /// 1D Array of all tiles in this chunk
    pub tiles: Vec<TileType>,
}

#[derive(Bundle, Clone)]
pub struct ChunkBundle<M: Material2d> {
    /// Parent bundle to help keep things clean!
    #[bundle]
    pub material: MaterialMesh2dBundle<M>,

    /// Chunk data
    pub chunk: Chunk,
}

impl Default for Chunk {
    fn default() -> Self {
        let mut tiles = Vec::with_capacity(CHUNK_SZ);
        tiles.resize(CHUNK_SZ, TileType::Dirt);

        Self { tiles }
    }
}

impl Chunk {
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[CHUNK_X * x + y] = tile;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        self.tiles[CHUNK_X * x + y]
    }

    pub fn create_mesh(&self) -> Mesh {
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
