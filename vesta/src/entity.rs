use crate::components::{Component, Transform};

/// Represents the very base object of the vesta engine
/// Entities contain components which describe their behavior
pub struct Entity {
    /// The transform of this entity (all entities have a transform)
    pub transform: Transform,

    /// A list of components that this entity contains
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            components: Vec::new(),
        }
    }

    pub fn add_component<T: 'static + Component>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }
}
