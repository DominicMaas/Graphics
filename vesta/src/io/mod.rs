mod keyboard;
mod mouse;

pub use keyboard::*;
pub use mouse::*;

pub struct IO {
    pub keyboard: Keyboard,
    pub mouse: Mouse
}