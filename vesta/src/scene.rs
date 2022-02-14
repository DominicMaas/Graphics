use bevy_ecs::world::World;

/// Represents a game scene. Contains a list of game objects
pub struct Scene {
    // Entities, components, and resources are stored within this world
    world: World,
}

impl Scene {
    pub fn default() -> Self {
        Self {
            world: World::default(),
        }
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.world
    }
}
