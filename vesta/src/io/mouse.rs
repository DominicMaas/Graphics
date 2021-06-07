use winit::event::{ElementState, MouseButton, WindowEvent};

#[derive(Clone)]
enum MouseAction {
    Pressed (MouseButton),
    Released (MouseButton),
}

#[derive(Clone)]
pub struct Mouse {
    position: cgmath::Vector2<f64>,
    actions: Vec<MouseAction>,
    held: [bool; 255],
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self { 
            position: (0.0, 0.0).into(),
            actions: vec!(),
            held: [false; 255],
        }
    }
    
    /// Returns true during the frame the user pressed the given mouse button.
    pub fn get_button_down(&self, button: MouseButton) -> bool {
        for action in &self.actions {
            if let &MouseAction::Pressed(value) = action {
                if value == button {
                    return true;
                }
            }
        }
            
        false
    }
    
    /// Returns true during the frame the user releases the given mouse button.
    pub fn get_button_up(&self, button: MouseButton) -> bool {
        for action in &self.actions {
            if let &MouseAction::Released(value) = action {
                if value == button {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Returns whether the given mouse button is held down.
    pub fn get_button(&self, button: MouseButton) -> bool {
        self.held[mouse_button_to_int(button)]
    }
    
    pub fn get_position(&self) -> cgmath::Vector2<f64> {
        self.position
    }
    
    pub(crate) fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.position = (position.x, position.y).into();
            },
            WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => {
                let button = *button;
                self.held[mouse_button_to_int(button)] = true;
                self.actions.push(MouseAction::Pressed(button));
            }
            WindowEvent::MouseInput { state: ElementState::Released, button, .. } => {
                let button = *button;
                self.held[mouse_button_to_int(button)] = false;
                self.actions.push(MouseAction::Released(button));
            }
            _ => {}
        }
    }
    
    /// This function clears events at the end of an update frame
    pub(crate) fn clear_events(&mut self) {
        self.actions = vec!();
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