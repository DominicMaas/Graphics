pub struct Mouse {
    pub(crate) mouse_position: cgmath::Vector2<f64>
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self { 
            mouse_position: (0.0, 0.0).into()
        }
    }
    
    /// Get the current mouse cursor position
    pub fn get_mouse_position(&self) -> cgmath::Vector2<f64> {
        self.mouse_position
    }
}