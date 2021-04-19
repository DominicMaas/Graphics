use winit::dpi::PhysicalSize;

/// Configuration for the vesta engine
pub struct Config {
    pub window_title: String,
    pub window_size: PhysicalSize<u32>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            window_title: "Vesta Engine".to_string(),
            window_size: PhysicalSize::new(800, 600),
        }
    }
}
