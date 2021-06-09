use std::f32::consts::PI;
use std::num::NonZeroU32;

//use noise::NoiseFn;
//use noise::OpenSimplex;
use rand::rngs::ThreadRng;
use rand::Rng;
use vesta::cgmath::{Matrix3, Matrix4, Quaternion, SquareMatrix, Vector2, Vector3};
use vesta::DrawMesh;

use crate::pixel::Pixel;
use crate::pixel::PixelType;

pub const CHUNK_SIZE: isize = 256;

pub const CHUNK_RENDER_SIZE: f32 = 2.0;

pub const GRAVITY: f32 = -10.0;

pub struct Chunk {
    pub position: Vector2<f32>,
    texture_mesh: vesta::Mesh,
    texture: vesta::wgpu::Texture,
    texture_bind_group: vesta::wgpu::BindGroup,
    uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
    data: Vec<Pixel>,
    color_buffer: Vec<u8>,
    loaded: bool,
    rng: ThreadRng,
    //noise: OpenSimplex,
    dirty: bool,
}

impl Chunk {
    pub fn new(renderer: &vesta::Renderer, position: Vector2<f32>) -> Self {
        // Simple square which the texture will be rendered onto
        let mut vertices = Vec::new();
        vertices.push(Self::create_vertex(1.0, 1.0, 1.0, 0.0)); // Top Right      1,1   0,1   0,0   1,0   1,1
        vertices.push(Self::create_vertex(1.0, -1.0, 0.0, 0.0)); // Bottom Right   1,0   1,1   0,1   0,0   1,0
        vertices.push(Self::create_vertex(-1.0, -1.0, 0.0, 1.0)); // Bottom Left    0,0   1,0   1,1   0,1   0,0
        vertices.push(Self::create_vertex(-1.0, 1.0, 1.0, 1.0)); // Top Left       0,1   0,0   1,0   1,1   0,1

        let texture_mesh = renderer.create_mesh(
            vertices,
            vec![
                0, 1, 3, // first triangle
                1, 2, 3, // second triangle
            ],
        );

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

        //let noise = OpenSimplex::new();

        Self {
            position,
            texture_mesh,
            texture,
            texture_bind_group,
            uniform_buffer,
            data,
            color_buffer: vec![0u8; (4 * CHUNK_SIZE * CHUNK_SIZE) as usize],
            loaded: false,
            rng,
            //noise,
            dirty: false,
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
                //let n = self
                //    .noise
                //    .get([(self.position.x as f64 + x as f64) / 0.0001, 0.0])
                //    * 64.0;
                //let yn = y as f64;

                let x_scale = 0.05;
                let y_scale = 5.5;
                let y_offset = 60.0;

                let gx = x as f32 + self.position.x;
                let gy = y as f32 + self.position.y;

                let r =
                    (((2.0 * gx * x_scale).sin() + (PI * gy * x_scale).sin()) * y_scale) + y_offset;

                if r >= y as f32 {
                    //e.g. n = 5
                    self.set_pixel_raw(x, y, Pixel::new(PixelType::Ground));
                } else {
                    self.set_pixel_raw(x, y, Pixel::new(PixelType::Air));
                }
            }
        }
    }

    pub fn add_snow(&mut self) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                if self.rng.gen_range(0..100) > 95 {
                    let p = self.get_pixel_raw(x, y).unwrap().get_type();
                    match p {
                        PixelType::Air => {
                            self.set_pixel_raw(x, y, Pixel::new(PixelType::Snow));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Write the raw data to the GPU via a texture
    fn write_to_gpu(&mut self, renderer: &vesta::Renderer) {
        // Write this buffer to the GPU
        renderer.queue.write_texture(
            vesta::wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: vesta::wgpu::Origin3d::ZERO,
            },
            self.color_buffer.as_slice(),
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

    #[inline(always)]
    fn get_pixel_raw(&mut self, x: isize, y: isize) -> Option<&mut Pixel> {
        if Self::pixel_in_bounds(x, y) == false {
            return None;
        }

        Some(&mut self.data[Self::pixel_index(x, y)])
    }

    /// Grab a read only version of the pixel
    pub fn get_pixel(&self, x: isize, y: isize) -> Option<Pixel> {
        if Self::pixel_in_bounds(x, y) == false {
            return None;
        }

        Some(self.data[Self::pixel_index(x, y)])
    }

    #[inline(always)]
    fn set_pixel_raw(&mut self, x: isize, y: isize, pixel: Pixel) {
        if Self::pixel_in_bounds(x, y) == false {
            return;
        }

        self.data[Self::pixel_index(x, y)] = pixel;

        pixel
            .get_color()
            .write_color_to_buffer(Self::pixel_index(x, y), &mut self.color_buffer);

        self.dirty = true;
    }

    fn create_model_matrix(position: Vector2<f32>) -> Matrix4<f32> {
        let rotation: Quaternion<f32> = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0))
            * Matrix4::from(rotation)
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

    pub fn update(&mut self, engine: &vesta::Engine) {
        let dt = engine.time.get_delta_time();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                match self.get_pixel_raw(x, y) {
                    Some(pixel) => match pixel.get_type() {
                        //PixelType::Ground => self.update_sand_old(x, y),
                        PixelType::Water => self.update_water(x, y, dt),
                        PixelType::Snow => self.update_sand_old(x, y),
                        PixelType::Sand => self.update_sand(x, y, dt),
                        _ => {}
                    },
                    None => {}
                }
            }
        }
    }

    fn update_sand(&mut self, x: isize, y: isize, dt: f32) {
        let pos = Vector2::new(x, y);
        let b_pos = Vector2::new(x, y - 1);
        let br_pos = Vector2::new(x + 1, y - 1);
        let bl_pos = Vector2::new(x - 1, y - 1);

        // Update the velocity
        let pixel = self.get_pixel_raw(x, y).unwrap();
        pixel.velocity.y = f32::clamp(pixel.velocity.y + (GRAVITY * dt), -10.0, 10.0);

        // Just check if can move directly below, if not, then reset velocity
        if !self.pixel_is(b_pos, PixelType::Air) {
            let pixel = self.get_pixel_raw(x, y).unwrap();
            pixel.velocity.y /= 2.0;
        }

        let pixel = self.get_pixel(x, y).unwrap();
        let v_pos = Vector2::new(x + pixel.velocity.x as isize, y + pixel.velocity.y as isize);

        // Physics
        if self.pixel_is(v_pos, PixelType::Air) || (self.pixel_is(v_pos, PixelType::Water)) {
            if self.pixel_is(v_pos, PixelType::Air) {
                self.swap_pixel_pos(pos, v_pos);
            }
        } else if self.pixel_is(b_pos, PixelType::Air) || self.pixel_is(b_pos, PixelType::Water) {
            let pixel = self.get_pixel_raw(x, y).unwrap();
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel_pos(pos, b_pos);
        } else if self.pixel_is(bl_pos, PixelType::Air) || self.pixel_is(bl_pos, PixelType::Water) {
            let pixel = self.get_pixel_raw(x, y).unwrap();
            pixel.velocity.x = 0.0;
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel_pos(pos, bl_pos);
        } else if self.pixel_is(br_pos, PixelType::Air) || self.pixel_is(br_pos, PixelType::Water) {
            let pixel = self.get_pixel_raw(x, y).unwrap();
            pixel.velocity.x = 0.0;
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel_pos(pos, br_pos);
        } else {
            // Do Nothing
        }

        //if let Some(p) = b_pixel {
        //    if p.get_type() == PixelType::Air {
        // TOOD
        //    }
        //}

        //if !self.pixel_empty(x, y - 1) {
        // If down empty
        ////    self.swap_pixel(x, y, x, y - 1);
        //    return;
        //}

        // TODO: Better way of inverting
        //if self.rng.gen_bool(0.5) {
        //if !self.pixel_empty(x - 1, y - 1) {
        // If down and left empty
        //    self.swap_pixel(x, y, x - 1, y - 1);
        //    return;
        // }

        //if !self.pixel_empty(x + 1, y - 1) {
        // If down and right empty
        //   self.swap_pixel(x, y, x + 1, y - 1);
        //   return;
        //}
        //} else {
        //    if !self.pixel_at(x + 1, y - 1) {
        // If down and right empty
        //        self.swap_pixel(x, y, x + 1, y - 1);
        //        return;
        //    }

        //   if !self.pixel_at(x - 1, y - 1) {
        // If down and left empty
        //      self.swap_pixel(x, y, x - 1, y - 1);
        //      return;
        //  }
        // }
    }

    fn update_sand_old(&mut self, x: isize, y: isize) {
        if !self.pixel_at(x, y - 1) {
            // If down empty
            self.swap_pixel(x, y, x, y - 1);
            return;
        }

        // TODO: Better way of inverting
        //if self.rng.gen_bool(0.5) {
        if !self.pixel_at(x - 1, y - 1) {
            // If down and left empty
            self.swap_pixel(x, y, x - 1, y - 1);
            return;
        }

        if !self.pixel_at(x + 1, y - 1) {
            // If down and right empty
            self.swap_pixel(x, y, x + 1, y - 1);
            return;
        }
        //} else {
        //    if !self.pixel_at(x + 1, y - 1) {
        // If down and right empty
        //        self.swap_pixel(x, y, x + 1, y - 1);
        //        return;
        //    }

        //   if !self.pixel_at(x - 1, y - 1) {
        // If down and left empty
        //      self.swap_pixel(x, y, x - 1, y - 1);
        //      return;
        //  }
        // }
    }

    fn update_water(&mut self, x: isize, y: isize, dt: f32) {
        // We know that this pixel exists
        let pixel = self.get_pixel_raw(x, y).unwrap();

        let fall_rate: u32 = 2;

        // Update the velocity
        pixel.velocity.y = f32::clamp(pixel.velocity.y + (GRAVITY * dt), -10.0, 10.0);

        //let yv = f32::clamp(pixel_velocity.y + (GRAVITY * dt), -10.0, 10.0);
        //pixel.set_velocity(Vector2::new(pixel_velocity.x, yv));

        if !self.pixel_at(x, y - 1) {
            // If down empty
            self.swap_pixel(x, y, x, y - 1);
        } else if !self.pixel_at(x - 1, y - 1) {
            // If down and left empty
            self.swap_pixel(x, y, x - 1, y - 1);
        } else if !self.pixel_at(x + 1, y - 1) {
            // If down and right empty
            self.swap_pixel(x, y, x + 1, y - 1);
        } else if !self.pixel_at(x - 1, y) {
            // If left empty
            self.swap_pixel(x, y, x - 1, y);
        } else if !self.pixel_at(x + 1, y) {
            // If right empty
            self.swap_pixel(x, y, x + 1, y);
        }
    }

    /// Swap a pixel from "from" to "to"
    pub fn swap_pixel(&mut self, from_x: isize, from_y: isize, to_x: isize, to_y: isize) {
        let from = self.get_pixel_raw(from_x, from_y).unwrap().clone();
        let to = self.get_pixel_raw(to_x, to_y).unwrap().clone();

        self.set_pixel_raw(from_x, from_y, to);
        self.set_pixel_raw(to_x, to_y, from);
    }

    pub fn swap_pixel_pos(&mut self, from: Vector2<isize>, to: Vector2<isize>) {
        let p_from = self.get_pixel_raw(from.x, from.y).unwrap().clone();
        let p_to = self.get_pixel_raw(to.x, to.y).unwrap().clone();

        self.set_pixel_raw(from.x, from.y, p_to);
        self.set_pixel_raw(to.x, to.y, p_from);
    }

    /// Overwrite a pixel at the specific index with a certain type. This will
    /// create a brand new pixel with a random color based on the type.
    pub fn overwrite_pixel(&mut self, x: isize, y: isize, pixel_type: PixelType) {
        self.set_pixel_raw(x, y, Pixel::new(pixel_type));
    }

    // OLD: Returns true if there is a pixel at this area (not air)
    fn pixel_at(&self, x: isize, y: isize) -> bool {
        let is_air_or_out_of_bounds = self.pixel_is(Vector2::new(x, y), PixelType::Air);
        return !is_air_or_out_of_bounds;
    }

    fn pixel_is(&self, pos: Vector2<isize>, pixel_type: PixelType) -> bool {
        match self.get_pixel(pos.x, pos.y) {
            Some(pixel) => pixel.get_type() == pixel_type,
            None => false,
        }
    }

    // ---------- Helpers ---------- //
    fn pixel_index(x: isize, y: isize) -> usize {
        CHUNK_SIZE as usize * x as usize + y as usize
    }

    fn pixel_in_bounds(x: isize, y: isize) -> bool {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || x < 0 || y < 0 {
            return false;
        }

        return true;
    }
}
