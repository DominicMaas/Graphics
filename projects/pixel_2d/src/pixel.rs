use rand::Rng;

// Represents a basic color
#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
                    a: 255,
                },
                PixelType::Water => Color {
                    r: 202,
                    g: 231,
                    b: 249,
                    a: 200,
                },
                PixelType::Ground => vary_color(Color {
                    r: 30,
                    g: 160,
                    b: 30,
                    a: 255,
                }),
                PixelType::Snow => vary_color(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                }),
                PixelType::Sand => vary_color(Color {
                    r: 194,
                    g: 178,
                    b: 128,
                    a: 255,
                }),
            },
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
    let val = (rand::thread_rng().gen_range(0..20) % 3) * 5;

    Color {
        r: color.r - val,
        g: color.g - val,
        b: color.b - val,
        a: 255,
    }
}
