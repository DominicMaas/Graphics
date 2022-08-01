use bevy::math::Vec2;
use rand::Rng;

// Represents a basic color
#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// A enum of all pixel types within the game
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum PixelType {
    #[default]
    Air,
    Water,
    Snow,
    Sand,
    Ground,
}

// Represents a pixel
#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    color: Color,
    pixel_type: PixelType,
    pub velocity: Vec2,
    pub updated_this_frame: bool,
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new(PixelType::Air)
    }
}

impl Pixel {
    /// Generate a new pixel of type
    pub fn new(pixel_type: PixelType) -> Self {
        Self {
            pixel_type,
            color: match pixel_type {
                PixelType::Air => Color {
                    r: 0,
                    g: 191,
                    b: 255,
                },
                PixelType::Water => Color {
                    r: 212,
                    g: 241,
                    b: 249,
                },
                PixelType::Ground => vary_color(Color {
                    r: 30,
                    g: 160,
                    b: 30,
                }),
                PixelType::Snow => vary_color(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                PixelType::Sand => vary_color(Color {
                    r: 194,
                    g: 178,
                    b: 128,
                }),
            },
            velocity: Vec2::new(0.0, 0.0),
            updated_this_frame: false,
        }
    }

    /// Get the type of this pixel
    pub fn get_type(&self) -> PixelType {
        self.pixel_type
    }

    /// Get the color of this pixel;
    pub fn get_color(&self) -> Color {
        self.color
    }
}

/// Vary the color of the provided color
fn vary_color(color: Color) -> Color {
    let mut rng = rand::thread_rng();
    let val = (rng.gen_range(0..20) % 3) * 5;

    Color {
        r: color.r - val,
        g: color.g - val,
        b: color.b - val,
    }
}

/*use rand::Rng;
use vesta::cgmath::Vector2;

use crate::chunk::CHUNK_SIZE;



impl Color {
    pub fn write_color_to_buffer(
        &self,
        i: usize,
        buffer: &mut Box<[u8; CHUNK_SIZE as usize * CHUNK_SIZE as usize * 4]>,
    ) {
        buffer[(i * 4)] = self.r;
        buffer[(i * 4) + 1] = self.g;
        buffer[(i * 4) + 2] = self.b;
        buffer[(i * 4) + 3] = 255;
    }
}



(/)*/
