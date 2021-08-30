pub struct Math {}

impl Math {
    /// Returns largest of two or more values.
    pub fn max(a: f32, b: f32) -> f32 {
        if a > b {
            a
        } else {
            b
        }
    }
}
