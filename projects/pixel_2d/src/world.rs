use crate::chunk::Chunk;

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new(renderer: &vesta::Renderer) -> Self {
        let mut chunks = Vec::new();
        chunks.push(Chunk::new(&renderer));

        for c in chunks.iter_mut() {
            c.load(renderer);
            c.rand_noise();
            c.write_to_gpu(renderer);
        }

        Self { chunks }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut vesta::wgpu::RenderPass<'a>) {
        for c in self.chunks.iter() {
            c.render(render_pass);
        }
    }

    pub fn rebuild(&mut self, renderer: &vesta::Renderer) {
        for c in self.chunks.iter_mut() {
            c.rand_noise();
            c.write_to_gpu(renderer);
        }
    }
}
