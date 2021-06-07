use winit::event::MouseButton;
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
    
    pub fn get_mouse_button_down(self, button: MouseButton) -> bool {
        false
    }
    
    /// This function clears events at the end of an update frame
    pub(crate) fn clear_events(&mut self) {
        
    }
}