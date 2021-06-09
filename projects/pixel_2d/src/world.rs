use std::time::Instant;

use vesta::cgmath::Vector2;

use crate::{
    chunk::{Chunk, CHUNK_RENDER_SIZE, CHUNK_SIZE},
    pixel::{Pixel, PixelType},
};

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new(renderer: &vesta::Renderer) -> Self {
        let mut chunks = Vec::new();
        chunks.push(Chunk::new(&renderer, Vector2::new(0.0, 0.0)));
        chunks.push(Chunk::new(
            &renderer,
            Vector2::new(CHUNK_RENDER_SIZE * 1.0, 0.0),
        ));
        chunks.push(Chunk::new(
            &renderer,
            Vector2::new(CHUNK_RENDER_SIZE * -1.0, 0.0),
        ));

        for c in chunks.iter_mut() {
            c.load(renderer);
            c.rand_noise();
            c.rebuild(renderer);
        }

        Self { chunks }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut vesta::wgpu::RenderPass<'a>) {
        //let now = Instant::now();

        for c in self.chunks.iter() {
            c.render(render_pass);
        }

        //println!("[World] [Draw] {}ms", now.elapsed().as_millis());
    }

    pub fn update(&mut self, engine: &vesta::Engine) {
        //let now = Instant::now();

        for c in self.chunks.iter_mut() {
            c.update(&engine);
        }

        //println!("[World] [Update] {}ms", now.elapsed().as_millis());
    }

    pub fn rebuild(&mut self, renderer: &vesta::Renderer) {
        //let now = Instant::now();

        for c in self.chunks.iter_mut() {
            c.rebuild(renderer);
        }

        //println!("[World] [Rebuild] {}ms", now.elapsed().as_millis());
    }

    pub fn add_snow(&mut self) {
        for c in self.chunks.iter_mut() {
            c.add_snow();
        }
    }

    pub fn get_pixel(&self, position: Vector2<f32>) -> Option<Pixel> {
        let range_min = -1.0;
        let range_max = 1.0;

        // Determine which chunk should receive this paint event
        for c in self.chunks.iter() {
            let new_pos = c.position - position;
            if new_pos.x > range_min
                && new_pos.x < range_max
                && new_pos.y > range_min
                && new_pos.y < range_max
            {
                // Convert to chunk coords
                let x = (((new_pos.x + 1.0) / 2.0) * CHUNK_SIZE as f32) as isize;
                let y = (((new_pos.y + 1.0) / 2.0) * CHUNK_SIZE as f32) as isize;

                return c.get_pixel(x, y);
            }
        }

        return None;
    }

    pub fn paint(&mut self, pixel_type: PixelType, radius: i32, position: Vector2<f32>) {
        let range_min = -1.0;
        let range_max = 1.0;

        let radius = radius as isize;

        // Determine which chunk should receive this paint event
        for c in self.chunks.iter_mut() {
            let new_pos = c.position - position;
            if new_pos.x > range_min
                && new_pos.x < range_max
                && new_pos.y > range_min
                && new_pos.y < range_max
            {
                // Convert to chunk coords
                let x_base = (((new_pos.x + 1.0) / 2.0) * CHUNK_SIZE as f32) as isize;
                let y_base = (((new_pos.y + 1.0) / 2.0) * CHUNK_SIZE as f32) as isize;

                if radius == 1 {
                    c.overwrite_pixel(x_base, y_base, pixel_type);
                } else {
                    for x in -radius..radius {
                        for y in -radius..radius {
                            if x * x + y * y <= radius * radius {
                                c.overwrite_pixel(x_base + x, y_base + y, pixel_type);
                            }
                        }
                    }
                }
            }
        }
    }
}
