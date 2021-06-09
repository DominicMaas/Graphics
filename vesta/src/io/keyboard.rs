use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

#[derive(Clone)]
enum KeyAction {
    Pressed(VirtualKeyCode),
    Released(VirtualKeyCode),
}

#[derive(Clone)]
pub struct Keyboard {
    actions: Vec<KeyAction>,
    held: [bool; 255],
}

impl Keyboard {
    pub(crate) fn new() -> Self {
        Self {
            actions: vec![],
            held: [false; 255],
        }
    }

    pub(crate) fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.held[keycode as usize] = true;
                            self.actions.push(KeyAction::Pressed(keycode));
                        }
                        ElementState::Released => {
                            self.held[keycode as usize] = false;
                            self.actions.push(KeyAction::Released(keycode));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Returns true during the frame the user starts pressing down the key identified by name.
    pub fn get_key_down(&self, key_code: VirtualKeyCode) -> bool {
        for action in &self.actions {
            if let &KeyAction::Pressed(value) = action {
                if value == key_code {
                    return true;
                }
            }
        }

        false
    }

    /// Returns true during the frame the user releases the key identified by name.
    pub fn get_key_up(&self, key_code: VirtualKeyCode) -> bool {
        for action in &self.actions {
            if let &KeyAction::Released(value) = action {
                if value == key_code {
                    return true;
                }
            }
        }

        false
    }

    /// Returns true while the user holds down the key identified by name.
    pub fn get_key(&self, key_code: VirtualKeyCode) -> bool {
        self.held[key_code as usize]
    }

    /// This function clears events at the end of an update frame
    pub(crate) fn clear_events(&mut self) {
        self.actions = vec![];
    }
}
