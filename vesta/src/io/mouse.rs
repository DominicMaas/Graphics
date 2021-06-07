use winit::event::{ElementState, MouseButton, WindowEvent};

#[derive(Clone)]
pub enum MouseAction {
    Pressed (usize),
    Released (usize),
}

#[derive(Clone)]
pub struct Mouse {
    // The current mouse cursor position
    mouse_position:     cgmath::Vector2<f64>,
    mouse_actions:      Vec<MouseAction>,
    mouse_held:         [bool; 255],
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self { 
            mouse_position:     (0.0, 0.0).into(),
            mouse_actions:      vec!(),
            mouse_held:         [false; 255],
        }
    }
    
    pub fn get_button_down(&self, button: MouseButton) -> bool {
        let button = mouse_button_to_int(button);
        
        for action in &self.mouse_actions {
            if let &MouseAction::Pressed(code) = action {
                if code == button {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn get_position(&self) -> cgmath::Vector2<f64> {
        self.mouse_position
    }
    
    pub(crate) fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x, position.y).into();
            },
            WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => {
                let button = mouse_button_to_int(*button);
                self.mouse_held[button] = true;
                self.mouse_actions.push(MouseAction::Pressed(button));
            }
            WindowEvent::MouseInput { state: ElementState::Released, button, .. } => {
                let button = mouse_button_to_int(*button);
                self.mouse_held[button] = false;
                self.mouse_actions.push(MouseAction::Released(button));
            }
            _ => {}
        }
    }
    
    /// This function clears events at the end of an update frame
    pub(crate) fn clear_events(&mut self) {
        self.mouse_actions = vec!();
    }
}

fn mouse_button_to_int(button: MouseButton) -> usize {
    match button {
        MouseButton::Left        => 0,
        MouseButton::Right       => 1,
        MouseButton::Middle      => 2,
        MouseButton::Other(byte) => byte as usize
    }
}