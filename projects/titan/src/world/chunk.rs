use vesta::{
    cgmath::{Matrix4, Vector2, Vector3},
    DrawMesh, Mesh,
};

use super::{
    BlockType, CHUNK_HEIGHT, CHUNK_WIDTH, FACE_BACK, FACE_BOTTOM, FACE_FRONT, FACE_LEFT,
    FACE_RIGHT, FACE_TOP, INDEX_MAP, NORMAL_MAP, VERTEX_MAP,
};

#[derive(Copy, Clone, Debug)]
pub enum ChunkState {
    /// The chunk has been created but there is no information associated with it
    Created,

    /// The chunk is currently loading (building terrian)
    Loading,

    /// The chunk is loaded and rendering as usual
    Loaded,

    /// The chunk is currently dirty and needs to be rebuilt
    Dirty,
}

pub struct Chunk {
    /// World position of this chunk
    position: Vector3<f32>,

    /// The mesh for the chunk
    mesh: Option<Mesh>,

    /// The model matrix telling the GPU how to render this chunk
    model_matrix: Matrix4<f32>,

    /// What state the chunks is in, this determines how this chunk is treated in the world
    state: ChunkState,

    /// 1D Array of all blocks in this chunk
    blocks: Vec<BlockType>,
}

impl Chunk {
    /// Create a new chunk, this only performs the bare minimum in order to maximise
    /// parallel processing later on
    pub fn new(position: Vector3<f32>) -> Self {
        Self {
            position,
            mesh: None,
            model_matrix: Matrix4::from_translation(position),
            state: ChunkState::Created,
            blocks: vec![0; (CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT) as usize],
        }
    }

    /// Loads the chunk and generates the expected terrian at this position
    pub fn load(&mut self) {
        self.state = ChunkState::Loading;

        // Populate the chunk
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    let _global_pos = Vector3::new(x as f32, y as f32, z as f32) + self.position;

                    // For now set everyting as 1
                    self.set_block(x, y, z, 1);
                }
            }
        }

        // Although the chunk has loaded, it's in a dirty state
        // (the chunk will not attempt to render a missing mesh)
        self.state = ChunkState::Dirty;
    }

    /// Rebuilds dirty chunks, this generates a mesh based on the current block data
    pub fn rebuild(&mut self, renderer: &vesta::Renderer) {
        // Determine if the chunk can be rebuilt
        let can_rebuild = match self.state {
            ChunkState::Created => {
                println!("Cannot rebuild chunk, it needs to be loaded first!");
                false
            }
            ChunkState::Loading => {
                println!("Cannot rebuild chunk, it's currently being loaded!");
                false
            }
            ChunkState::Loaded => {
                println!("Cannot rebuild chunk, it's not dirt! (use the set_dirty() function)");
                false
            }
            _ => true,
        };

        if !can_rebuild {
            return;
        }

        let mut vertices: Vec<vesta::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut curr_index: u32 = 0;

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    let block_type = self.get_block(x, y, z);
                    if block_type == 0 {
                        continue;
                    }

                    // Front Face
                    if true {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                VERTEX_MAP[FACE_FRONT][i],
                                NORMAL_MAP[FACE_FRONT][i],
                                Vector2::new(0.0, 0.0),
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_FRONT][i])
                        }

                        curr_index += 4;
                    }

                    // Back Face
                    if false {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                VERTEX_MAP[FACE_BACK][i],
                                NORMAL_MAP[FACE_BACK][i],
                                Vector2::new(0.0, 0.0),
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_BACK][i])
                        }

                        curr_index += 4;
                    }

                    // Left Face
                    if false {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                VERTEX_MAP[FACE_LEFT][i],
                                NORMAL_MAP[FACE_LEFT][i],
                                Vector2::new(0.0, 0.0),
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_LEFT][i])
                        }

                        curr_index += 4;
                    }

                    // Right Face
                    if false {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                VERTEX_MAP[FACE_RIGHT][i],
                                NORMAL_MAP[FACE_RIGHT][i],
                                Vector2::new(0.0, 0.0),
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_RIGHT][i])
                        }

                        curr_index += 4;
                    }

                    // Top Face
                    if false {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                VERTEX_MAP[FACE_TOP][i],
                                NORMAL_MAP[FACE_TOP][i],
                                Vector2::new(0.0, 0.0),
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_TOP][i])
                        }

                        curr_index += 4;
                    }

                    // Bottom Face
                    if false {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                VERTEX_MAP[FACE_BOTTOM][i],
                                NORMAL_MAP[FACE_BOTTOM][i],
                                Vector2::new(0.0, 0.0),
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_BOTTOM][i])
                        }

                        curr_index += 4;
                    }
                }
            }
        }

        self.mesh = Some(Mesh::new(vertices, indices, &renderer.device));
        self.state = ChunkState::Loaded;
    }

    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        _engine: &vesta::Engine,
    ) {
        // Only render if there is a mesh and the chunk is in the correct state
        let render = match self.mesh {
            Some(_) => match self.state {
                ChunkState::Dirty | ChunkState::Loaded => true,
                _ => false,
            },
            None => false,
        };

        if !render {
            return;
        }

        // TODO: Bind
        //render_pass.set_bind_group(1, &self.camera.uniform_buffer.bind_group, &[]);

        render_pass.draw_mesh(self.mesh.as_ref().unwrap());
    }

    // ----- Block Array Helpers ----- //

    /// Set the block type at the provided position and mark the chunk as dirty
    pub fn set_block(&mut self, x: u32, y: u32, z: u32, block_type: BlockType) {
        match self.state {
            ChunkState::Dirty | ChunkState::Loaded | ChunkState::Loading => {
                self.blocks[(z * CHUNK_WIDTH * CHUNK_HEIGHT + y * CHUNK_WIDTH + x) as usize] =
                    block_type;
                self.state = ChunkState::Dirty;
            }
            _ => {
                println!(
                    "Cannot set block at position ({},{},{}), the chunk is not in a loaded state!",
                    x, y, z
                );
            }
        }
    }

    /// Get the block type of the block at the specified location
    pub fn get_block(&self, x: u32, y: u32, z: u32) -> BlockType {
        match self.state {
            ChunkState::Dirty | ChunkState::Loaded => {
                self.blocks[(z * CHUNK_WIDTH * CHUNK_HEIGHT + y * CHUNK_WIDTH + x) as usize]
            }
            _ => {
                println!(
                    "Cannot get block at position ({},{},{}), the chunk is not in a loaded state!",
                    x, y, z
                );
                0
            }
        }
    }

    pub fn get_state(&self) -> ChunkState {
        self.state
    }
}
