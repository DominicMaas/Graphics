use rand::Rng;

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    color: Color,
    pixel_type: PixelType,
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new()
    }
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            pixel_type: PixelType::Air,
            color: Color {
                r: 0,
                g: 191,
                b: 255,
            },
        }
    }

    pub fn set(&mut self, pixel_type: PixelType) {
        self.pixel_type = pixel_type;
        self.color = match pixel_type {
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
        }
    }

    pub fn get_type(&self) -> PixelType {
        self.pixel_type
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelType {
    Air,
    Water,
    Snow,
    Ground,
}

fn vary_color(color: Color) -> Color {
    let mut rng = rand::thread_rng();
    let val = (rng.gen_range(0..20) % 3) * 5;

    Color {
        r: color.r - val,
        g: color.g - val,
        b: color.b - val,
    }
}
