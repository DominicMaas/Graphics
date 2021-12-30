pub mod game_object;
pub mod light;
pub mod transform;

pub use game_object::*;
pub use light::*;
pub use transform::*;

pub trait Component {
    /// The editor readable name for a component
    fn get_name() -> &'static str
    where
        Self: Sized;
}
