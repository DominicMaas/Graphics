use std::num::NonZeroU32;

use noise::NoiseFn;
use noise::OpenSimplex;
use rand::rngs::ThreadRng;
use rand::Rng;
use vesta::cgmath::{Matrix3, Matrix4, Quaternion, SquareMatrix, Vector2, Vector3};
use vesta::DrawMesh;

use crate::pixel::Pixel;
use crate::pixel::PixelType;

pub const CHUNK_SIZE: isize = 256;

pub const CHUNK_RENDER_SIZE: f32 = 2.0;

pub struct Chunk {
    pub position: Vector2<f32>,
    texture_mesh: vesta::Mesh,
    texture: vesta::wgpu::Texture,
    texture_bind_group: vesta::wgpu::BindGroup,
    uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
    data: Vec<Pixel>,
    loaded: bool,
    rng: ThreadRng,
    noise: OpenSimplex,
    dirty: bool,
}

impl Chunk {
    pub fn new(renderer: &vesta::Renderer, position: Vector2<f32>) -> Self {
        // Simple square which the texture will be rendered onto
        let mut vertices = Vec::new();
        vertices.push(Self::create_vertex( 1.0,  1.0, 1.0, 0.0)); // Top Right      1,1   0,1   0,0   1,0   1,1
        vertices.push(Self::create_vertex( 1.0, -1.0, 0.0, 0.0)); // Bottom Right   1,0   1,1   0,1   0,0   1,0
        vertices.push(Self::create_vertex(-1.0, -1.0, 0.0, 1.0)); // Bottom Left    0,0   1,0   1,1   0,1   0,0
        vertices.push(Self::create_vertex(-1.0,  1.0, 1.0, 1.0)); // Top Left       0,1   0,0   1,0   1,1   0,1

        let texture_mesh = renderer.create_mesh(vertices, vec![
            0, 1, 3, // first triangle
            1, 2, 3  // second triangle
        ]);

        // Uniform for adjusting the position of this chunk in the world
        let model = Self::create_model_matrix(position);
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
            depth_or_array_layers: 1,
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
                mag_filter: vesta::wgpu::FilterMode::Nearest,
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

        let data = vec![Pixel::default(); (CHUNK_SIZE * CHUNK_SIZE) as usize];
        let rng = rand::thread_rng();

        let noise = OpenSimplex::new();
        
        Self {
            position,
            texture_mesh,
            texture,
            texture_bind_group,
            uniform_buffer,
            data,
            loaded: false,
            rng,
            noise,
            dirty: false
        }
    }

    pub fn load(&mut self, _renderer: &vesta::Renderer) {
        if self.loaded {
            return;
        }

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
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let n = self.noise.get([(self.position.x as f64 + x as f64) / 0.001, 0.0]) * 32.0;
                let yn = y as f64;
                
                //let s = self.rng.gen_range(0..100);
                let p = self.get_pixel(x, y).unwrap();
                if n > yn {
                    p.set(PixelType::Ground)
                }
            }
        }
        
        self.dirty = true;
    }
    
    pub fn add_snow(&mut self) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let s = self.rng.gen_range(0..100);
                let p = self.get_pixel(x, y).unwrap();
                match p.get_type() {
                    PixelType::Air => {
                        if s > 95 {
                            p.set(PixelType::Snow)
                        }
                    }
                    _ => {}
                }
            }
        }
        
        self.dirty = true;
    }
    
    /// Write the raw data to the GPU via a texture
    fn write_to_gpu(&self, renderer: &vesta::Renderer) {
        // Create a buffer of the pixel colors
        let mut buffer = vec![0u8; (4 * CHUNK_SIZE * CHUNK_SIZE) as usize];
        for i in 0..self.data.len() {
            let color = self.data[i].get_color();
            buffer[(i * 4)] = color.r;
            buffer[(i * 4) + 1] = color.g;
            buffer[(i * 4) + 2] = color.b;
            buffer[(i * 4) + 3] = 255;
        }

        // Write this buffer to the GPU
        renderer.queue.write_texture(
            vesta::wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: vesta::wgpu::Origin3d::ZERO,
            },
            buffer.as_slice(),
            vesta::wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new((4 * CHUNK_SIZE) as u32),
                rows_per_image: NonZeroU32::new(CHUNK_SIZE as u32),
            },
            vesta::wgpu::Extent3d {
                width: CHUNK_SIZE as u32,
                height: CHUNK_SIZE as u32,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn get_pixel(&mut self, x: isize, y: isize) -> Option<&mut Pixel> {
        if x >= CHUNK_SIZE as isize || x < 0 {
            return None;
        }
        
        if y >= CHUNK_SIZE as isize || y < 0 {
            return None;
        }
        
        Some(&mut self.data[(CHUNK_SIZE * x + y) as usize])
    }

    fn create_model_matrix(position: Vector2<f32>) -> Matrix4<f32> {
        let rotation: Quaternion<f32> = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0)) * Matrix4::from(rotation)
    }

    fn create_vertex(x: f32, y: f32, u: f32, v: f32) -> vesta::Vertex {
        vesta::Vertex {
            position: Vector3::new(x, y, 0.0),
            color: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(u, v),
            normal: Vector3::new(0.0, 0.0, 0.0),
        }
    }
    
    pub fn rebuild(&mut self, renderer: &vesta::Renderer) {
        if self.dirty {
            self.write_to_gpu(renderer);
            self.dirty = false;
        } 
    }
    
    pub fn update(&mut self) {     
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                match self.get_pixel(x, y) {
                    Some(pixel) => match pixel.get_type() {
                        PixelType::Ground => self.update_sand(x, y),
                        PixelType::Water => self.update_water(x, y),
                        PixelType::Snow => self.update_sand(x, y),
                        _ => {}
                    },
                    None => {}
                }
            }
        } 
    }
    
    fn update_sand(&mut self, x: isize, y: isize)  {                     
        if !self.pixel_at(x, y - 1) {  // If down empty        
            self.swap_pixel(x, y, x, y - 1);
            return;
        } 
        
        // TODO: Better way of inverting
        if self.rng.gen_bool(0.5) {
            if !self.pixel_at(x - 1, y - 1) {  // If down and left empty        
                self.swap_pixel(x, y, x - 1, y - 1);
                return
            } 
            
            if !self.pixel_at(x + 1, y - 1) {  // If down and right empty        
                self.swap_pixel(x, y, x + 1, y - 1);
                return
            }
        } else {
            if !self.pixel_at(x + 1, y - 1) {  // If down and right empty        
                self.swap_pixel(x, y, x + 1, y - 1);
                return
            }
            
            if !self.pixel_at(x - 1, y - 1) {  // If down and left empty        
                self.swap_pixel(x, y, x - 1, y - 1);
                return
            } 
        }
    }
    
    fn update_water(&mut self, x: isize, y: isize) {                     
        if !self.pixel_at(x, y - 1) {  // If down empty        
            self.swap_pixel(x, y, x, y - 1);
        } else if !self.pixel_at(x - 1, y - 1) {  // If down and left empty        
            self.swap_pixel(x, y, x - 1, y - 1);
        } else if !self.pixel_at(x + 1, y - 1) {  // If down and right empty        
            self.swap_pixel(x, y, x + 1, y - 1);
        } else if !self.pixel_at(x - 1, y) {  // If left empty        
            self.swap_pixel(x, y, x - 1, y);
        } else if !self.pixel_at(x + 1, y) {  // If right empty        
            self.swap_pixel(x, y, x + 1, y);
        }
    }
    
    pub fn swap_pixel(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
        let from_pixel_type = self.get_pixel(from_x, from_y).unwrap().get_type();
        let to_pixel_type = self.get_pixel(to_x, to_y).unwrap().get_type();
                
        self.get_pixel(to_x, to_y).unwrap().set(from_pixel_type);
        self.get_pixel(from_x, from_y).unwrap().set(to_pixel_type); 
        
        self.dirty = true;
    }
    
    pub fn overwrite_pixel(&mut self, x: isize, y: isize, pixel_type: PixelType) {
        match self.get_pixel(x, y) {
            Some(p) => {
                p.set(pixel_type);
                self.dirty = true;
            },
            None => {}
        }
    }
    
    fn pixel_at(&mut self, x: isize, y: isize) -> bool {
        let pixel = self.get_pixel(x, y);
        match pixel {
            Some(pixel) => {
                match pixel.get_type() {
                    PixelType::Air => false,
                    _ => true
                }
            },
            None => true,
        }
    }
}
