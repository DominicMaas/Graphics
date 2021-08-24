use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};

#[derive(Clone)]
enum MouseAction {
    Pressed(MouseButton),
    Released(MouseButton),
}

#[derive(Clone)]
pub struct Mouse {
    position: cgmath::Vector2<f64>,
    position_previous: cgmath::Vector2<f64>,
    delta: cgmath::Vector2<f64>,
    actions: Vec<MouseAction>,
    held: [bool; 255],
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self {
            position: (0.0, 0.0).into(),
            position_previous: (0.0, 0.0).into(),
            delta: (0.0, 0.0).into(),
            actions: vec![],
            held: [false; 255],
        }
    }

    /// Returns true during the frame the user pressed the given mouse button.
    pub fn get_button_down(&self, button: MouseButton) -> bool {
        for action in &self.actions {
            if let MouseAction::Pressed(value) = *action {
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
            if let MouseAction::Released(value) = *action {
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

    pub fn get_position_f32(&self) -> cgmath::Vector2<f32> {
        cgmath::Vector2::new(self.position.x as f32, self.position.y as f32)
    }

    pub fn get_delta(&self) -> cgmath::Vector2<f64> {
        self.delta
    }

    pub fn get_delta_f32(&self) -> cgmath::Vector2<f32> {
        cgmath::Vector2::new(self.delta.x as f32, self.delta.y as f32)
    }

    pub(crate) fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.position = (position.x, position.y).into();
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                let button = *button;
                self.held[mouse_button_to_int(button)] = true;
                self.actions.push(MouseAction::Pressed(button));
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                let button = *button;
                self.held[mouse_button_to_int(button)] = false;
                self.actions.push(MouseAction::Released(button));
            }
            _ => {}
        }
    }

    pub(crate) fn handle_device_event(&mut self, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.delta = (delta.0, delta.1).into();
        }
    }

    /// This function clears events at the end of an update frame
    pub(crate) fn clear_events(&mut self) {
        self.actions = vec![];
        self.delta = cgmath::vec2(0.0, 0.0);
        self.position_previous = self.position;
    }
}

fn mouse_button_to_int(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(byte) => byte as usize,
    }
}
