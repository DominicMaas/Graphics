use vesta::{
    cgmath::{Matrix3, Matrix4, Quaternion, Vector2, Vector3},
    DrawMesh, Mesh,
};

use crate::world::Generator;

use super::{
    BlockType, CHUNK_HEIGHT, CHUNK_WIDTH, FACE_BACK, FACE_BOTTOM, FACE_FRONT, FACE_LEFT,
    FACE_RIGHT, FACE_TOP, INDEX_MAP, NORMAL_MAP, TEXTURE_MAP, TEX_X_STEP, VERTEX_MAP,
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

    /// What state the chunks is in, this determines how this chunk is treated in the world
    state: ChunkState,

    /// 1D Array of all blocks in this chunk
    blocks: Vec<BlockType>,

    /// Tells the GPU how to render the object
    uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,

    /// Tells the chunk how to construct itself
    generator: Generator,
}

impl Chunk {
    /// Create a new chunk, this only performs the bare minimum in order to maximise
    /// parallel processing later on
    pub fn new(position: Vector3<f32>, seed: u32, renderer: &vesta::Renderer) -> Self {
        let rotation: Quaternion<f32> = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        let model = Matrix4::from_translation(position) * Matrix4::from(rotation);
        let normal = Matrix3::from_cols(model.x.truncate(), model.y.truncate(), model.z.truncate());

        let uniform_data = vesta::ModelUniform { model, normal };
        let uniform_buffer = vesta::UniformBuffer::new(
            "C-Body Uniform Buffer",
            vesta::wgpu::ShaderStage::VERTEX,
            uniform_data,
            &renderer.device,
        );

        let generator = Generator::new(seed, 0.0, 0, 0.0, 0.0);

        Self {
            position,
            mesh: None,
            state: ChunkState::Created,
            blocks: vec![0; (CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT) as usize],
            uniform_buffer,
            generator,
        }
    }

    /// Loads the chunk and generates the expected terrian at this position
    pub fn load(&mut self) {
        self.state = ChunkState::Loading;

        // Populate the chunk
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    let global_pos = Vector3::new(x as f32, y as f32, z as f32) + self.position;
                    let block_type = self.generator.get_theoretical_block_type(global_pos);
                    self.set_block(x, y, z, block_type);
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
                    let pos = Vector3::new(x as f32, y as f32, z as f32);

                    // We need signed integers for transparency checks
                    let ix = x as i32;
                    let iy = y as i32;
                    let iz = z as i32;

                    let block_type = self.get_block(x, y, z);
                    if block_type == 0 {
                        continue;
                    }

                    // TODO: Fancy Texture map thing (maybe from file?)
                    let front_tex = Vector2::new(TEX_X_STEP * 1.0, 0.0);
                    let back_tex = Vector2::new(TEX_X_STEP * 1.0, 0.0);
                    let left_tex = Vector2::new(TEX_X_STEP * 1.0, 0.0);
                    let right_tex = Vector2::new(TEX_X_STEP * 1.0, 0.0);
                    let top_tex = Vector2::new(TEX_X_STEP * 2.0, 0.0);
                    let bottom_tex = Vector2::new(TEX_X_STEP * 0.0, 0.0);

                    // Front Face
                    if self.is_transparent(ix, iy, iz + 1) {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                pos + VERTEX_MAP[FACE_FRONT][i],
                                NORMAL_MAP[FACE_FRONT][i],
                                front_tex + TEXTURE_MAP[FACE_FRONT][i],
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_FRONT][i])
                        }

                        curr_index += 4;
                    }

                    // Back Face
                    if self.is_transparent(ix, iy, iz - 1) {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                pos + VERTEX_MAP[FACE_BACK][i],
                                NORMAL_MAP[FACE_BACK][i],
                                back_tex + TEXTURE_MAP[FACE_BACK][i],
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_BACK][i])
                        }

                        curr_index += 4;
                    }

                    // Left Face
                    if self.is_transparent(ix - 1, iy, iz) {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                pos + VERTEX_MAP[FACE_LEFT][i],
                                NORMAL_MAP[FACE_LEFT][i],
                                left_tex + TEXTURE_MAP[FACE_LEFT][i],
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_LEFT][i])
                        }

                        curr_index += 4;
                    }

                    // Right Face
                    if self.is_transparent(ix + 1, iy, iz) {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                pos + VERTEX_MAP[FACE_RIGHT][i],
                                NORMAL_MAP[FACE_RIGHT][i],
                                right_tex + TEXTURE_MAP[FACE_RIGHT][i],
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_RIGHT][i])
                        }

                        curr_index += 4;
                    }

                    // Top Face
                    if self.is_transparent(ix, iy + 1, iz) {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                pos + VERTEX_MAP[FACE_TOP][i],
                                NORMAL_MAP[FACE_TOP][i],
                                top_tex + TEXTURE_MAP[FACE_TOP][i],
                            ));
                        }

                        for i in 0..6 {
                            indices.push(curr_index + INDEX_MAP[FACE_TOP][i])
                        }

                        curr_index += 4;
                    }

                    // Bottom Face
                    if self.is_transparent(ix, iy - 1, iz) {
                        for i in 0..4 {
                            vertices.push(vesta::Vertex::with_tex_coords(
                                pos + VERTEX_MAP[FACE_BOTTOM][i],
                                NORMAL_MAP[FACE_BOTTOM][i],
                                bottom_tex + TEXTURE_MAP[FACE_BOTTOM][i],
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

        self.mesh = Some(renderer.create_mesh(vertices, indices));
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

        render_pass.set_bind_group(1, &self.uniform_buffer.bind_group, &[]);
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

    fn get_block_type(&self, x: i32, y: i32, z: i32) -> BlockType {
        // Above the max possible chunk
        if y >= CHUNK_HEIGHT as i32 {
            return 0;
        }

        // Outside of this chunk
        if (x < 0) || (z < 0) || (x >= CHUNK_WIDTH as i32) || (z >= CHUNK_WIDTH as i32) {
            // TODO: Check for existing chunks

            // This chunk is not loaded / does not exist, get the theoretical block type
            let mut world_pos = Vector3::new(x as f32, y as f32, z as f32);
            world_pos += self.position;

            return self.generator.get_theoretical_block_type(world_pos);
        }

        // Get the block type within the chunk
        return self.get_block(x as u32, y as u32, z as u32);
    }

    pub fn get_state(&self) -> ChunkState {
        self.state
    }

    fn is_transparent(&self, x: i32, y: i32, z: i32) -> bool {
        // Never render the bottom face of the world
        if y < 0 {
            return false;
        }

        return self.get_block_type(x, y, z) == 0;
    }
}
