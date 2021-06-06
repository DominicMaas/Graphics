pub struct Mouse {
    // The current mouse cursor position
    pub mouse_position: cgmath::Vector2<f64>
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self { 
            mouse_position: (0.0, 0.0).into()
        }
    }
}