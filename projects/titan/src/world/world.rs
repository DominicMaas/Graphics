use vesta::cgmath::Vector3;

use super::{Chunk, CHUNK_HEIGHT, CHUNK_WIDTH};

pub struct World {
    chunks: Vec<Chunk>,
    block_map_texture: vesta::Texture,
    pub rendered_chunks: usize,
}

impl World {
    pub fn new(renderer: &vesta::Renderer, seed: u32) -> Self {
        // Build the chunks
        let mut chunks = Vec::new();
        for x in 0..8 {
            for z in 0..8 {
                let chunk = Chunk::new(
                    Vector3::new(
                        (x as i32 * CHUNK_WIDTH as i32) as f32,
                        0.0,
                        (z as i32 * CHUNK_WIDTH as i32) as f32,
                    ),
                    seed,
                    &renderer,
                ); // Temp
                chunks.push(chunk);
            }
        }

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
        }
    }

    pub fn update(&mut self, renderer: &vesta::Renderer) {
        // Process Chunks
        for chunk in self.chunks.iter_mut() {
            match chunk.get_state() {
                crate::world::ChunkState::Created => chunk.load(),
                crate::world::ChunkState::Dirty => chunk.rebuild(&renderer),
                _ => {}
            }
        }
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
