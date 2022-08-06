use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::ImageSampler,
    },
};
use rand::Rng;

use crate::pixel::{Pixel, PixelType};

pub const CHUNK_WIDTH: usize = 64;
pub const CHUNK_HEIGHT: usize = 256;

#[derive(Component)]
pub struct Chunk {
    // Raw pixel data
    pixels: Vec<Pixel>,

    // If this flag is set, the chunk texture is out of date
    dirty: bool,
}

impl Chunk {
    /// Checks if the provided coordinates are
    /// in bounds of this chunk
    fn pixel_in_bounds(pos: Vec2) -> bool {
        if pos.x >= CHUNK_WIDTH as f32 || pos.y >= CHUNK_HEIGHT as f32 || pos.x < 0.0 || pos.y < 0.0
        {
            return false;
        }

        return true;
    }

    /// Helper method for mapping a 2D pixel coordinate to a 1D
    /// index
    #[inline(always)]
    fn pixel_index(x: usize, y: usize) -> usize {
        CHUNK_WIDTH * y + x
    }

    /// Get the mutable pixel at the specified coordinate
    fn get_pixel_mut(&mut self, pos: Vec2) -> Option<&mut Pixel> {
        if Self::pixel_in_bounds(pos) == false {
            return None;
        }

        Some(&mut self.pixels[Self::pixel_index(pos.x as usize, pos.y as usize)])
    }

    /// Grab a read only version of the pixel
    pub fn get_pixel(&self, pos: Vec2) -> Option<Pixel> {
        if Self::pixel_in_bounds(pos) == false {
            return None;
        }

        Some(self.pixels[Self::pixel_index(pos.x as usize, pos.y as usize)])
    }

    /// Set the pixel at a specified position
    pub fn set_pixel(&mut self, pos: Vec2, pixel: Pixel) {
        if Self::pixel_in_bounds(pos) == false {
            return;
        }

        self.pixels[Self::pixel_index(pos.x as usize, pos.y as usize)] = pixel;
        self.dirty = true;
    }

    /// Swap a pixel from "from" to "to"
    pub fn swap_pixel(&mut self, from: Vec2, to: Vec2) {
        let p_from = self.get_pixel_mut(from).unwrap().clone();
        let p_to = self.get_pixel_mut(to).unwrap().clone();

        self.set_pixel(from, p_to);
        self.set_pixel(to, p_from);
    }

    /// Overwrite a pixel at the specific index with a certain type. This will
    /// create a brand new pixel with a random color based on the type.
    pub fn overwrite_pixel(&mut self, pos: Vec2, pixel_type: PixelType) {
        self.set_pixel(pos, Pixel::new(pixel_type));
    }

    /// Return if the pixels at the specified position is of a specified
    /// type
    fn pixel_is(&self, pos: Vec2, pixel_type: PixelType) -> bool {
        match self.get_pixel(pos) {
            Some(pixel) => pixel.get_type() == pixel_type,
            None => false,
        }
    }

    /// If a pixel can move to this location via automa
    fn can_move(&self, pos: Vec2) -> bool {
        match self.get_pixel(pos) {
            Some(pixel) => pixel.get_type() == PixelType::Air,
            None => false,
        }
    }
}

pub fn setup_chunks(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    for i in -4..4 {
        let pixels = vec![Pixel::default(); (CHUNK_WIDTH * CHUNK_HEIGHT) as usize];

        let default_data = vec![0; CHUNK_WIDTH * CHUNK_HEIGHT];
        let mut image = Image::new_fill(
            Extent3d {
                width: CHUNK_WIDTH as u32,
                height: CHUNK_HEIGHT as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &default_data,
            TextureFormat::Rgba8UnormSrgb,
        );

        image.sampler_descriptor = ImageSampler::nearest();

        commands
            .spawn_bundle(SpriteBundle {
                texture: images.add(image),
                sprite: Sprite {
                    flip_y: true,
                    ..Default::default()
                },
                transform: Transform::from_xyz(CHUNK_WIDTH as f32 * i as f32, 0.0, 0.0),
                ..Default::default()
            })
            .insert(Chunk {
                pixels,
                dirty: true,
            });
    }
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
                let color = p.get_color();

                image.data[i] = color.r;
                image.data[i + 1] = color.g;
                image.data[i + 2] = color.b;
                image.data[i + 3] = color.a;

                i += 4;
            }

            chunk.dirty = false
        }
    }
}

pub fn update_chunks(mut query: Query<&mut Chunk>) {
    for mut chunk in query.iter_mut() {
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                let pos = Vec2::new(x as f32, y as f32);

                match chunk.get_pixel_mut(pos) {
                    Some(pixel) => match pixel.get_type() {
                        PixelType::Water => update_water(&mut chunk, pos),
                        PixelType::Snow => update_sand(&mut chunk, pos),
                        PixelType::Sand => update_sand(&mut chunk, pos),
                        _ => {}
                    },
                    None => {}
                }
            }
        }
    }
}

fn get_rand_pos(chunk: &mut Chunk, pos: Vec2, x_offset: f32) -> Option<Vec2> {
    let left = pos - Vec2::new(1.0 + x_offset, 0.0);
    let right = pos + Vec2::new(1.0 + x_offset, 0.0);

    let left_free = chunk.can_move(left);
    let right_free = chunk.can_move(right);

    if left_free && right_free {
        let ran = rand::thread_rng().gen_range(0..2);
        if ran == 0 {
            return Some(left);
        } else {
            return Some(right);
        }
    } else if left_free {
        return Some(left);
    } else if right_free {
        return Some(right);
    }

    return None;
}

fn update_sand(chunk: &mut Chunk, pos: Vec2) {
    let below_pos = pos - Vec2::new(0.0, 1.0);

    if chunk.can_move(below_pos) {
        chunk.swap_pixel(pos, below_pos);
        return;
    } else if let Some(new_pos) = get_rand_pos(chunk, below_pos, 0.0) {
        chunk.swap_pixel(pos, new_pos);
        return;
    }
}

fn update_water(chunk: &mut Chunk, pos: Vec2) {
    let b_pos = Vec2::new(pos.x, pos.y - 1.0);

    // Go straight down
    if chunk.pixel_is(b_pos, PixelType::Air) {
        chunk.swap_pixel(pos, b_pos);
    } else {
        // Go random below left or right
        if let Some(new_pos) = get_rand_pos(chunk, b_pos, 0.0) {
            chunk.swap_pixel(pos, new_pos);
        } else if let Some(new_pos) = get_rand_pos(chunk, pos, 0.0) {
            chunk.swap_pixel(pos, new_pos);
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

    to
*/
