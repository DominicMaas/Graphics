use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::pixel::{Pixel, PixelType};

pub const CHUNK_SIZE: usize = 256;

#[derive(Component)]
pub struct Chunk {
    // Raw pixel data
    pixels: Vec<Pixel>,

    // If this flag is set, the chunk texture is out of date
    dirty: bool,
}

impl Chunk {
    #[inline(always)]
    fn pixel_in_bounds(x: usize, y: usize) -> bool {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || x < 0 || y < 0 {
            return false;
        }

        return true;
    }

    #[inline(always)]
    fn pixel_index(x: usize, y: usize) -> usize {
        CHUNK_SIZE * x + y
    }

    #[inline(always)]
    fn get_pixel_raw(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if Self::pixel_in_bounds(x, y) == false {
            return None;
        }

        Some(&mut self.pixels[Self::pixel_index(x, y)])
    }

    /// Grab a read only version of the pixel
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Pixel> {
        if Self::pixel_in_bounds(x, y) == false {
            return None;
        }

        Some(self.pixels[Self::pixel_index(x, y)])
    }

    #[inline(always)]
    fn set_pixel_raw(&mut self, x: usize, y: usize, pixel: Pixel) {
        if Self::pixel_in_bounds(x, y) == false {
            return;
        }

        self.pixels[Self::pixel_index(x, y)] = pixel;
        self.dirty = true;
    }

    pub fn rand_noise(&mut self) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                //let gx = self.position.x + x as f32;
                //let gy = self.position.y + y as f32;

                //let r = self.noise.get_noise(gx, gy) * 250.0;

                self.set_pixel_raw(x, y, Pixel::new(PixelType::Air));

                //if r >= y as f32 {
                //self.set_pixel_raw(x, y, Pixel::new(PixelType::Ground));
                //} else {
                //self.set_pixel_raw(x, y, Pixel::new(PixelType::Air));
                //}
            }
        }
    }
}

pub fn setup_chunks(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let pixels = vec![Pixel::default(); (CHUNK_SIZE * CHUNK_SIZE) as usize];

    let default_data = vec![50; CHUNK_SIZE * CHUNK_SIZE];
    let image = Image::new_fill(
        Extent3d {
            width: CHUNK_SIZE as u32,
            height: CHUNK_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &default_data,
        TextureFormat::Rgba8UnormSrgb,
    );

    let image_handle = images.add(image);

    commands
        .spawn_bundle(SpriteBundle {
            texture: image_handle,
            ..Default::default()
        })
        .insert(Chunk {
            pixels,
            dirty: true,
        });
}

// This system is in charge of updating the textures of dirty chunks on the GPU
pub fn update_chunk_textures_system(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut Chunk, &Handle<Image>)>,
) {
    for (mut chunk, handle) in query.iter_mut() {
        if chunk.dirty {
            let image = images
                .get_mut(handle)
                .expect("Chunk should have a chunk texture!");

            let mut i = 0;
            for p in chunk.pixels.iter() {
                image.data[i] = p.get_color().r;
                image.data[i + 1] = p.get_color().g;
                image.data[i + 2] = p.get_color().b;
                image.data[i + 3] = 255;

                i += 4;
            }

            chunk.dirty = false
        }
    }
}

/*use std::mem::ManuallyDrop;
use std::num::NonZeroU32;

use rand::rngs::ThreadRng;
use rand::Rng;
use vesta::cgmath::{Matrix3, Matrix4, Quaternion, SquareMatrix, Vector2, Vector3};
use vesta::DrawMesh;

use crate::pixel::Pixel;
use crate::pixel::PixelType;

pub const CHUNK_SIZE: isize = 256;

pub const CHUNK_RENDER_SIZE: f32 = 2.0;

pub const GRAVITY: f32 = -9.8;

pub struct Chunk {
    pub position: Vector2<f32>,
    texture_mesh: vesta::Mesh,
    texture: vesta::wgpu::Texture,
    texture_bind_group: vesta::wgpu::BindGroup,
    uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
    data: Vec<Pixel>,
    color_buffer: Box<[u8; CHUNK_SIZE as usize * CHUNK_SIZE as usize * 4]>, //Vec<u8>,
    loaded: bool,
    rng: ThreadRng,
    dirty: bool,
}

impl Chunk {
    pub fn new(renderer: &vesta::Renderer, position: Vector2<f32>, _seed: u64) -> Self {
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
            vesta::wgpu::ShaderStages::VERTEX,
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
                usage: vesta::wgpu::TextureUsages::TEXTURE_BINDING
                    | vesta::wgpu::TextureUsages::COPY_DST,
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

        Self {
            position,
            texture_mesh,
            texture,
            texture_bind_group,
            uniform_buffer,
            data,
            color_buffer: Self::create_color_buffer(),
            loaded: false,
            rng,
            dirty: false,
        }
    }

    /// Helper function to create the color buffer
    fn create_color_buffer() -> Box<[u8; CHUNK_SIZE as usize * CHUNK_SIZE as usize * 4]> {
        let mut data = ManuallyDrop::new(vec![0; CHUNK_SIZE as usize * CHUNK_SIZE as usize * 4]);
        unsafe {
            Box::from_raw(
                data.as_mut_ptr() as *mut [u8; CHUNK_SIZE as usize * CHUNK_SIZE as usize * 4]
            )
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
                //let gx = self.position.x + x as f32;
                //let gy = self.position.y + y as f32;

                //let r = self.noise.get_noise(gx, gy) * 250.0;

                self.set_pixel_raw(x, y, Pixel::new(PixelType::Air));

                //if r >= y as f32 {
                //self.set_pixel_raw(x, y, Pixel::new(PixelType::Ground));
                //} else {
                //self.set_pixel_raw(x, y, Pixel::new(PixelType::Air));
                //}
            }
        }
    }

    pub fn add_snow(&mut self) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                if self.rng.gen_range(0..=100) > 95 {
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
                aspect: vesta::wgpu::TextureAspect::All,
            },
            self.color_buffer.as_mut(),
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
        if Self::pixel_in_bounds(Vector2::new(x, y)) == false {
            return None;
        }

        Some(&mut self.data[Self::pixel_index(x, y)])
    }

    /// Grab a read only version of the pixel
    pub fn get_pixel(&self, x: isize, y: isize) -> Option<Pixel> {
        if Self::pixel_in_bounds(Vector2::new(x, y)) == false {
            return None;
        }

        Some(self.data[Self::pixel_index(x, y)])
    }

    #[inline(always)]
    fn set_pixel_raw(&mut self, x: isize, y: isize, pixel: Pixel) {
        if Self::pixel_in_bounds(Vector2::new(x, y)) == false {
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
                let pos = Vector2::new(x, y);

                match self.get_pixel_raw(x, y) {
                    Some(pixel) => match pixel.get_type() {
                        PixelType::Water => self.update_water(pos, dt),
                        PixelType::Snow => self.update_sand(pos, dt),
                        PixelType::Sand => self.update_sand(pos, dt),
                        _ => {}
                    },
                    None => {}
                }
            }
        }
    }

    fn update_sand(&mut self, pos: Vector2<isize>, dt: f32) {
        let ran = self.rng.gen_range(0..=1);
        let offset = if ran == 0 { 1 } else { -1 };

        let b_pos = Vector2::new(pos.x, pos.y - 1);
        let br_pos = Vector2::new(pos.x + offset, pos.y - 1);
        let bl_pos = Vector2::new(pos.x + -offset, pos.y - 1);

        // Update the velocity
        let pixel = self.get_pixel_raw(pos.x, pos.y).unwrap();
        pixel.velocity.y = f32::clamp(pixel.velocity.y + (GRAVITY * dt), -10.0, 10.0);

        // Just check if can move directly below, if not, then reset velocity
        if !self.pixel_is(b_pos, PixelType::Air) {
            self.get_pixel_raw(pos.x, pos.y).unwrap().velocity.y /= 2.0;
        }

        let pixel = self.get_pixel(pos.x, pos.y).unwrap();
        let v_pos = Vector2::new(
            pos.x + pixel.velocity.x as isize,
            pos.y + pixel.velocity.y as isize,
        );

        // Physics
        if self.pixel_is(v_pos, PixelType::Air) || (self.pixel_is(v_pos, PixelType::Water)) {
            if self.pixel_is(v_pos, PixelType::Air) {
                self.swap_pixel(pos, v_pos);
            }
        } else if self.pixel_is(b_pos, PixelType::Air) || self.pixel_is(b_pos, PixelType::Water) {
            let pixel = self.get_pixel_raw(pos.x, pos.y).unwrap();
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel(pos, b_pos);
        } else if self.pixel_is(bl_pos, PixelType::Air) || self.pixel_is(bl_pos, PixelType::Water) {
            let pixel = self.get_pixel_raw(pos.x, pos.y).unwrap();
            pixel.velocity.x = 0.0;
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel(pos, bl_pos);
        } else if self.pixel_is(br_pos, PixelType::Air) || self.pixel_is(br_pos, PixelType::Water) {
            let pixel = self.get_pixel_raw(pos.x, pos.y).unwrap();
            pixel.velocity.x = 0.0;
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel(pos, br_pos);
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

    fn update_water(&mut self, pos: Vector2<isize>, dt: f32) {
        let fall_rate = 2;
        let spread_rate = 5;

        let ran = self.rng.gen_range(0..=1);
        let spread_rate_right = if ran == 0 { spread_rate } else { -spread_rate };
        let spread_rate_left = -spread_rate_right;

        let b_pos = Vector2::new(pos.x, pos.y - fall_rate);
        let br_pos = Vector2::new(pos.x + spread_rate_right, pos.y - fall_rate);
        let bl_pos = Vector2::new(pos.x + spread_rate_left, pos.y - fall_rate);
        let _r_pos = Vector2::new(pos.x + spread_rate_right, pos.y);
        let _l_pos = Vector2::new(pos.x - spread_rate_left, pos.y);

        // Update the velocity
        let pixel = self.get_pixel_raw(pos.x, pos.y).unwrap();
        pixel.velocity.y = f32::clamp(pixel.velocity.y + (GRAVITY * dt), -10.0, 10.0);

        // Just check if can move directly below, if not, then reset velocity
        if !self.pixel_is(b_pos, PixelType::Air) {
            let pixel = self.get_pixel_raw(pos.x, pos.y).unwrap();
            pixel.velocity.y /= 2.0;
        }

        let pixel = self.get_pixel(pos.x, pos.y).unwrap();
        let v_pos = Vector2::new(
            pos.x + pixel.velocity.x as isize,
            pos.y + pixel.velocity.y as isize,
        );

        if self.pixel_is(v_pos, PixelType::Air) {
            self.swap_pixel(pos, v_pos);
        } else if self.pixel_is(b_pos, PixelType::Air) {
            let pixel = self.get_pixel_raw(b_pos.x, b_pos.y).unwrap();
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel(pos, b_pos);
        } else if self.pixel_is(bl_pos, PixelType::Air) {
            let x_vel = if let None = self.pixel_in_type(pos, PixelType::Water) {
                if self.rng.gen_range(0..=1) == 0 {
                    -1.0
                } else {
                    1.0
                }
            } else {
                0.0
            };

            let pixel = self.get_pixel_raw(bl_pos.x, bl_pos.y).unwrap();
            pixel.velocity.x = x_vel;
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel(pos, bl_pos);
        } else if self.pixel_is(br_pos, PixelType::Air) {
            let x_vel = if let None = self.pixel_in_type(pos, PixelType::Water) {
                if self.rng.gen_range(0..=1) == 0 {
                    -1.0
                } else {
                    1.0
                }
            } else {
                0.0
            };

            let pixel = self.get_pixel_raw(br_pos.x, br_pos.y).unwrap();
            pixel.velocity.x = x_vel;
            pixel.velocity.y += GRAVITY * dt;

            self.swap_pixel(pos, br_pos);
        } else if self.rng.gen_range(0..=10) == 0 {
            // In water, sometimes move around?
            if let Some(water_pos) = self.pixel_in_type(pos, PixelType::Water) {
                self.swap_pixel(pos, water_pos);
            }
        } else {
            // Don't try to spread if something directly above
            if !self.pixel_completely_surrounded(pos) {
                for i in 0..fall_rate {
                    for j in (0..spread_rate).rev() {
                        let x_minus_j_y_plus_i = Vector2::new(pos.x - j, pos.y + i);
                        let x_plus_j_y_plus_i = Vector2::new(pos.x + j, pos.y + i);

                        if self.pixel_is(x_minus_j_y_plus_i, PixelType::Air) {
                            self.swap_pixel(pos, x_minus_j_y_plus_i);
                        }

                        if self.pixel_is(x_plus_j_y_plus_i, PixelType::Air) {
                            self.swap_pixel(pos, x_plus_j_y_plus_i);
                        }
                    }
                }
            }
        }
    }

    /// Swap a pixel from "from" to "to"
    pub fn swap_pixel(&mut self, from: Vector2<isize>, to: Vector2<isize>) {
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

    fn pixel_is(&self, pos: Vector2<isize>, pixel_type: PixelType) -> bool {
        match self.get_pixel(pos.x, pos.y) {
            Some(pixel) => pixel.get_type() == pixel_type,
            None => false,
        }
    }

    /// Returns none if the pixel is not in the given type, otherwise returns the
    /// position in which the pixel is touching the type
    fn pixel_in_type(&self, pos: Vector2<isize>, pixel_type: PixelType) -> Option<Vector2<isize>> {
        if self.pixel_is(pos, pixel_type) {
            return Some(pos);
        }

        if self.pixel_is(pos + Vector2::new(0, -1), pixel_type) {
            return Some(pos + Vector2::new(0, -1));
        }

        if self.pixel_is(pos + Vector2::new(0, 1), pixel_type) {
            return Some(pos + Vector2::new(0, 1));
        }

        if self.pixel_is(pos + Vector2::new(-1, 0), pixel_type) {
            return Some(pos + Vector2::new(-1, 0));
        }

        if self.pixel_is(pos + Vector2::new(-1, -1), pixel_type) {
            return Some(pos + Vector2::new(-1, -1));
        }

        if self.pixel_is(pos + Vector2::new(-1, 1), pixel_type) {
            return Some(pos + Vector2::new(-1, 1));
        }

        if self.pixel_is(pos + Vector2::new(1, 0), pixel_type) {
            return Some(pos + Vector2::new(1, 0));
        }

        if self.pixel_is(pos + Vector2::new(1, -1), pixel_type) {
            return Some(pos + Vector2::new(1, -1));
        }

        if self.pixel_is(pos + Vector2::new(1, 1), pixel_type) {
            return Some(pos + Vector2::new(1, 1));
        }

        return None;
    }

    fn pixel_completely_surrounded(&self, pos: Vector2<isize>) -> bool {
        // Top
        if Self::pixel_in_bounds(pos + Vector2::new(1, 0))
            && self.pixel_is(pos + Vector2::new(1, 0), PixelType::Air)
        {
            return false;
        }

        // Bottom
        if Self::pixel_in_bounds(pos + Vector2::new(-1, 0))
            && self.pixel_is(pos + Vector2::new(-1, 0), PixelType::Air)
        {
            return false;
        }

        // Left
        if Self::pixel_in_bounds(pos + Vector2::new(-1, 0))
            && self.pixel_is(pos + Vector2::new(-1, 0), PixelType::Air)
        {
            return false;
        }

        // Right
        if Self::pixel_in_bounds(pos + Vector2::new(1, 0))
            && self.pixel_is(pos + Vector2::new(1, 0), PixelType::Air)
        {
            return false;
        }

        // Top Left
        if Self::pixel_in_bounds(pos + Vector2::new(-1, -1))
            && self.pixel_is(pos + Vector2::new(-1, -1), PixelType::Air)
        {
            return false;
        }

        // Top Right
        if Self::pixel_in_bounds(pos + Vector2::new(1, -1))
            && self.pixel_is(pos + Vector2::new(1, -1), PixelType::Air)
        {
            return false;
        }

        // Bottom Left
        if Self::pixel_in_bounds(pos + Vector2::new(-1, 1))
            && self.pixel_is(pos + Vector2::new(-1, 1), PixelType::Air)
        {
            return false;
        }

        // Bottom Right
        if Self::pixel_in_bounds(pos + Vector2::new(1, 1))
            && self.pixel_is(pos + Vector2::new(1, 1), PixelType::Air)
        {
            return false;
        }

        return true;
    }

    // ---------- Helpers ---------- //
    fn pixel_index(x: isize, y: isize) -> usize {
        CHUNK_SIZE as usize * x as usize + y as usize
    }

    fn pixel_in_bounds(pos: Vector2<isize>) -> bool {
        if pos.x >= CHUNK_SIZE || pos.y >= CHUNK_SIZE || pos.x < 0 || pos.y < 0 {
            return false;
        }

        return true;
    }
}
*/
