use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Perlin, Seedable, OpenSimplex, Fbm, NoiseFn};
use rand::rngs::ThreadRng;
use rand::Rng;
use vesta::cgmath::num_traits::AsPrimitive;
use vesta::cgmath::{Matrix3, Matrix4, Quaternion, SquareMatrix, Vector2, Vector3};
use vesta::DrawMesh;

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Pixel {
    color: Color,
}

unsafe impl vesta::bytemuck::Zeroable for Pixel {}
unsafe impl vesta::bytemuck::Pod for Pixel {}

pub const CHUNK_SIZE: usize = 512;

pub struct Chunk {
    texture_mesh: vesta::Mesh,
    texture: vesta::wgpu::Texture,
    texture_bind_group: vesta::wgpu::BindGroup,
    uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
    data: Vec<Pixel>,
    loaded: bool,
    rng: ThreadRng,
}

impl Chunk {
    pub fn new(renderer: &vesta::Renderer) -> Self {
        // Simple square which the texture will be rendered onto
        let mut vertices = Vec::new();
        vertices.push(Self::create_vertex(-1.0, -1.0, 0.0, 0.0)); // Top Left
        vertices.push(Self::create_vertex(1.0, -1.0, 1.0, 0.0)); // Top Right
        vertices.push(Self::create_vertex(-1.0, 1.0, 0.0, 1.0)); // Bottom Left
        vertices.push(Self::create_vertex(1.0, 1.0, 1.0, 1.0)); // Bottom Right

        let texture_mesh = renderer.create_mesh(vertices, vec![0, 2, 3, 3, 1, 0]);

        // Uniform for adjusting the position of this chunk in the world
        let model = Self::create_model_matrix();
        let uniform_data = vesta::ModelUniform {
            model,
            normal: Matrix3::identity(),
        };
        let uniform_buffer = vesta::UniformBuffer::new(
            "Chunk Uniform Buffer",
            vesta::wgpu::ShaderStage::VERTEX,
            uniform_data,
            &renderer.device,
        );

        // Texture that data will be written to
        let texture_size = vesta::wgpu::Extent3d {
            width: CHUNK_SIZE as u32,
            height: CHUNK_SIZE as u32,
            depth: 1,
        };

        let texture = renderer
            .device
            .create_texture(&vesta::wgpu::TextureDescriptor {
                label: Some("Chunk Texture"),
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: vesta::wgpu::TextureDimension::D2,
                format: vesta::wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: vesta::wgpu::TextureUsage::SAMPLED | vesta::wgpu::TextureUsage::COPY_DST,
            });

        let view = texture.create_view(&vesta::wgpu::TextureViewDescriptor::default());
        let sampler = renderer
            .device
            .create_sampler(&vesta::wgpu::SamplerDescriptor {
                address_mode_u: vesta::wgpu::AddressMode::ClampToEdge,
                address_mode_v: vesta::wgpu::AddressMode::ClampToEdge,
                address_mode_w: vesta::wgpu::AddressMode::ClampToEdge,
                mag_filter: vesta::wgpu::FilterMode::Linear,
                min_filter: vesta::wgpu::FilterMode::Nearest,
                mipmap_filter: vesta::wgpu::FilterMode::Nearest,
                ..Default::default()
            });

        // Create the appropriate bind group for the input data
        let texture_bind_group =
            renderer
                .device
                .create_bind_group(&vesta::wgpu::BindGroupDescriptor {
                    layout: &vesta::Texture::create_bind_group_layout(&renderer.device),
                    entries: &[
                        vesta::wgpu::BindGroupEntry {
                            binding: 0,
                            resource: vesta::wgpu::BindingResource::TextureView(&view),
                        },
                        vesta::wgpu::BindGroupEntry {
                            binding: 1,
                            resource: vesta::wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                    label: Some("texture_bind_group"),
                });

        let data = vec![Pixel::default(); CHUNK_SIZE * CHUNK_SIZE];
        let rng = rand::thread_rng();

        Self {
            texture_mesh,
            texture,
            texture_bind_group,
            uniform_buffer,
            data,
            loaded: false,
            rng,
        }
    }

    pub fn load(&mut self, renderer: &vesta::Renderer) {
        if self.loaded {
            return;
        }

        let mut p = self.get_pixel(0, 0);
        p.color.g = 255;

        let mut p2 = self.get_pixel(511, 511);
        p2.color.r = 255;

        self.write_to_gpu(renderer);
        self.loaded = true;
    }

    pub fn render<'a>(&'a self, render_pass: &mut vesta::wgpu::RenderPass<'a>) {
        if self.loaded {
            render_pass.set_bind_group(1, &self.uniform_buffer.bind_group, &[]);
            render_pass.set_bind_group(2, &self.texture_bind_group, &[]);
            render_pass.draw_mesh(&self.texture_mesh);
        }
    }

    pub fn rand_noise(&mut self) {


       // let noise = OpenSimplex::new().set_seed(s);
       // println!("S:{}", noise.seed());

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let s = self.rng.gen_range(0..100);
                let mut p = self.get_pixel(x, y);

               // let n = noise.get([x as f64 / 256.0, y as f64 / 256.0]);
                if s > 50 {
                    p.color.r = 255;
                    p.color.g = 0;
                    p.color.b = 0;
                } else {
                    p.color.r = 255;
                    p.color.g = 255;
                    p.color.b = 0;
                }
            }
        }
    }

    /// Write the raw data to the GPU via a texture
    pub fn write_to_gpu(&self, renderer: &vesta::Renderer) {
        // Create a buffer of the pixel colors
        let mut buffer = vec![0u8; 4 * CHUNK_SIZE * CHUNK_SIZE];
        for i in 0..self.data.len() {
            let color = self.data[i].color;
            buffer[(i * 4)] = color.r;
            buffer[(i * 4) + 1] = color.g;
            buffer[(i * 4) + 2] = color.b;
            buffer[(i * 4) + 3] = 255;
        }

        // Write this buffer to the GPU
        renderer.queue.write_texture(
            vesta::wgpu::TextureCopyView {
                texture: &self.texture,
                mip_level: 0,
                origin: vesta::wgpu::Origin3d::ZERO,
            },
            buffer.as_slice(),
            vesta::wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: (4 * CHUNK_SIZE) as u32,
                rows_per_image: CHUNK_SIZE as u32,
            },
            vesta::wgpu::Extent3d {
                width: CHUNK_SIZE as u32,
                height: CHUNK_SIZE as u32,
                depth: 1,
            },
        );
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.data[CHUNK_SIZE * x + y]
    }

    fn create_model_matrix() -> Matrix4<f32> {
        let rotation: Quaternion<f32> = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from(rotation)
    }

    fn create_vertex(x: f32, y: f32, u: f32, v: f32) -> vesta::Vertex {
        vesta::Vertex {
            position: Vector3::new(x, y, 0.0),
            color: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(u, v),
            normal: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
