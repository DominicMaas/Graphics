use vesta::cgmath::Vector3;

use super::{Chunk, CHUNK_HEIGHT, CHUNK_WIDTH};

const CREATE_PER_FRAME: u32 = 5;
const LOAD_PER_FRAME: u32 = 2;
const REBUILD_PER_FRAME: u32 = 2;

const RENDER_DISTANCE: u32 = 4; // 4 chunks

pub struct World {
    chunks: Vec<Chunk>,
    block_map_texture: vesta::Texture,
    pub rendered_chunks: usize,
    seed: u32,

    created_this_frame: u32,
    loaded_this_frame: u32,
    rebuilt_this_frame: u32,
}

impl World {
    pub fn new(renderer: &vesta::Renderer, seed: u32) -> Self {
        // Build the chunks
        let chunks = Vec::new();
        let block_map_texture = renderer
            .create_texture_from_bytes(
                include_bytes!("../res/img/block_map.png"),
                Some("res/img/block_map.png"),
                vesta::TextureConfig {
                    sampler_mag_filter: vesta::wgpu::FilterMode::Nearest,
                    sampler_min_filter: vesta::wgpu::FilterMode::Nearest,
                    sampler_mipmap_filter: vesta::wgpu::FilterMode::Nearest,
                    ..Default::default()
                },
            )
            .unwrap();

        Self {
            chunks,
            block_map_texture,
            rendered_chunks: 0,
            seed,
            created_this_frame: 0,
            loaded_this_frame: 0,
            rebuilt_this_frame: 0,
        }
    }

    pub fn update(&mut self, renderer: &vesta::Renderer, camera: &vesta::Camera) {
        self.created_this_frame = 0;
        self.loaded_this_frame = 0;
        self.rebuilt_this_frame = 0;

        // Process Chunks
        for chunk in self.chunks.iter_mut() {
            match chunk.get_state() {
                crate::world::ChunkState::Created => {
                    if self.loaded_this_frame <= LOAD_PER_FRAME {
                        chunk.load();
                        self.loaded_this_frame += 1;
                    }
                }
                crate::world::ChunkState::Dirty => {
                    if self.rebuilt_this_frame <= REBUILD_PER_FRAME {
                        chunk.rebuild(&renderer);
                        self.rebuilt_this_frame += 1;
                    }
                }
                _ => {}
            }
        }

        // Generate new chunks (memory alocation)
        let render_distance = (RENDER_DISTANCE * CHUNK_WIDTH) as i32;

        // Calculation about the camera position and render distance
        let center_x = ((f32::floor(camera.position.x / CHUNK_WIDTH as f32) * CHUNK_WIDTH as f32)
            - CHUNK_WIDTH as f32) as i32;
        let center_z = ((f32::floor(camera.position.z / CHUNK_WIDTH as f32) * CHUNK_WIDTH as f32)
            - CHUNK_WIDTH as f32) as i32;

        let step = CHUNK_WIDTH as usize;

        for x in (center_x - render_distance..center_x + render_distance).step_by(step) {
            for z in (center_z - render_distance..center_z + render_distance).step_by(step) {
                if !self.chunk_at(Vector3::new(x as f32, 0.0, z as f32)) {
                    self.chunks.push(Chunk::new(
                        Vector3::new(x as f32, 0.0, z as f32),
                        self.seed,
                        &renderer,
                    ));
                    self.created_this_frame += 1;
                    println!(
                        "[{}] Building chunk at {}, 0, {}",
                        self.created_this_frame, x, z
                    );
                }

                if self.created_this_frame > CREATE_PER_FRAME {
                    break;
                }
            }

            if self.created_this_frame > CREATE_PER_FRAME {
                break;
            }
        }

        // Generate new chunks
        //for (float x = cWorldX - renderDistance; x <= cWorldX + renderDistance; x += CHUNK_WIDTH)
        //for (float z = cWorldZ - renderDistance; z <= cWorldZ + renderDistance; z += CHUNK_WIDTH) {
        //    if (findChunk(glm::vec3(x, 0, z)) == NULL) {
        //        _chunks.push_back(new Chunk(glm::vec3(x, 0, z), this));
        //    }
        //}
    }

    fn chunk_at(&self, position: Vector3<f32>) -> bool {
        for chunk in self.chunks.iter() {
            let chunk_position = chunk.get_position();

            if (position.x >= chunk_position.x)
                && (position.z >= chunk_position.z)
                && (position.x < chunk_position.x + CHUNK_WIDTH as f32)
                && (position.z < chunk_position.z + CHUNK_WIDTH as f32)
            {
                return true;
            }
        }

        false
    }

    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        engine: &vesta::Engine,
        camera: &vesta::Camera,
    ) {
        render_pass.set_bind_group(2, &self.block_map_texture.bind_group.as_ref().unwrap(), &[]);

        let frustum = vesta::Frustum::new(camera.projection.calc_matrix() * camera.calc_matrix());

        self.rendered_chunks = 0;

        for chunk in self.chunks.iter_mut() {
            if frustum.is_box_visible(
                chunk.get_position(),
                chunk.get_position()
                    + Vector3::new(CHUNK_WIDTH as f32, CHUNK_HEIGHT as f32, CHUNK_WIDTH as f32),
            ) {
                chunk.render(render_pass, engine);
                self.rendered_chunks += 1;
            }
        }
    }
}
