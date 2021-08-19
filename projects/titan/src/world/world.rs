use std::{collections::VecDeque, time::Instant};

use vesta::cgmath::Vector3;

use crate::world::Generator;

use super::{Chunk, CHUNK_HEIGHT, CHUNK_WIDTH};

#[cfg(debug_assertions)]
const CREATE_PER_FRAME: u32 = 15;

#[cfg(not(debug_assertions))]
const CREATE_PER_FRAME: u32 = 25;

#[cfg(debug_assertions)]
const LOAD_PER_FRAME: u32 = 2;

#[cfg(not(debug_assertions))]
const LOAD_PER_FRAME: u32 = 20;

#[cfg(debug_assertions)]
const REBUILD_PER_FRAME: u32 = 6;

#[cfg(not(debug_assertions))]
const REBUILD_PER_FRAME: u32 = 15;

#[cfg(debug_assertions)]
const RENDER_DISTANCE: u32 = 6;

#[cfg(not(debug_assertions))]
const RENDER_DISTANCE: u32 = 8;

const CREATE_DISTANCE: u32 = RENDER_DISTANCE * 2;
const DELETE_DISTANCE: u32 = CREATE_DISTANCE + 4;

pub struct World {
    chunks: Vec<Chunk>,
    block_map_texture: vesta::Texture,
    pub rendered_chunks: usize,
    generator: Generator,

    created_this_frame: u32,
    loaded_this_frame: u32,
    rebuilt_this_frame: u32,
}

impl World {
    pub fn new(renderer: &vesta::Renderer, seed: u64) -> Self {
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

        let generator = Generator::new(seed);

        Self {
            chunks,
            block_map_texture,
            rendered_chunks: 0,
            generator,
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
                    if self.loaded_this_frame < LOAD_PER_FRAME {
                        chunk.load(&self.generator);
                        self.loaded_this_frame += 1;
                    }
                }
                crate::world::ChunkState::Dirty => {
                    if self.rebuilt_this_frame < REBUILD_PER_FRAME {
                        chunk.rebuild(&renderer, &self.generator);
                        self.rebuilt_this_frame += 1;
                    }
                }
                _ => {}
            }
        }

        let create_now = Instant::now();

        // Generate new chunks (memory location)
        let create_distance = (CREATE_DISTANCE * CHUNK_WIDTH) as i32;

        // Calculation about the camera position and render distance
        let center_x = ((f32::floor(camera.position.x / CHUNK_WIDTH as f32) * CHUNK_WIDTH as f32)
            - CHUNK_WIDTH as f32) as i32;
        let center_z = ((f32::floor(camera.position.z / CHUNK_WIDTH as f32) * CHUNK_WIDTH as f32)
            - CHUNK_WIDTH as f32) as i32;

        let step = CHUNK_WIDTH as usize;

        for x in (center_x - create_distance..center_x + create_distance).step_by(step) {
            for z in (center_z - create_distance..center_z + create_distance).step_by(step) {
                if !self.chunk_at(Vector3::new(x as f32, 0.0, z as f32)) {
                    self.chunks
                        .push(Chunk::new(Vector3::new(x as f32, 0.0, z as f32), &renderer));
                    self.created_this_frame += 1;
                }

                if self.created_this_frame >= CREATE_PER_FRAME {
                    break;
                }
            }

            if self.created_this_frame > CREATE_PER_FRAME {
                break;
            }
        }

        // Delete old chunks (TODO: Save to disk or something in the future)
        self.chunks.retain(|chunk| {
            let in_bounds = f32::abs(chunk.center_position().x - camera.position.x)
                < (CHUNK_WIDTH * DELETE_DISTANCE) as f32
                && f32::abs(chunk.center_position().z - camera.position.z)
                    < (CHUNK_WIDTH * DELETE_DISTANCE) as f32;

            in_bounds
        });
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
            // If the chunk is in the players view distance, continue
            if f32::abs(chunk.center_position().x - camera.position.x)
                < (CHUNK_WIDTH * RENDER_DISTANCE) as f32
                && f32::abs(chunk.center_position().z - camera.position.z)
                    < (CHUNK_WIDTH * RENDER_DISTANCE) as f32
            {
                // If the chunk is in the frustum, continue
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
}
