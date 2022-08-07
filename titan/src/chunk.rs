use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Chunk constants

pub const CHUNK_XZ: usize = 16;
pub const CHUNK_Y: usize = 32;
pub const CHUNK_SZ: usize = CHUNK_XZ * CHUNK_Y;

pub const WORLD_XZ: isize = 4;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum VoxelType {
    #[default]
    Air,
    Dirt,
}

/// Represents a single chunk in the world
#[derive(Component)]
pub struct Chunk {
    /// 1D Array of all blocks in this chunk
    pub blocks: Vec<VoxelType>,
}

#[derive(Default, Bundle)]
pub struct ChunkBundle {
    /// Chunk data
    pub chunk: Chunk,
    /// The chunk material (this is standard)
    pub material: Handle<StandardMaterial>,
    /// Where the chunk is located in the world
    pub transform: Transform,
    /// Global world transform
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for Chunk {
    fn default() -> Self {
        let mut blocks = Vec::with_capacity(CHUNK_SZ);
        blocks.resize(CHUNK_SZ, VoxelType::Dirt);
        Self { blocks }
    }
}

pub fn chunk_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk_mat = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    for x in -WORLD_XZ..WORLD_XZ {
        for z in -WORLD_XZ..WORLD_XZ {
            commands
                .spawn_bundle(ChunkBundle {
                    material: chunk_mat.clone(),
                    transform: Transform::from_xyz(
                        (x * CHUNK_XZ as isize) as f32,
                        0.0,
                        (z * CHUNK_XZ as isize) as f32,
                    ),
                    ..Default::default()
                })
                .insert(meshes.add(Mesh::from(shape::Cube {
                    size: CHUNK_XZ as f32,
                })))
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(
                    CHUNK_XZ as f32 / 2.0,
                    CHUNK_XZ as f32 / 2.0,
                    CHUNK_XZ as f32 / 2.0,
                ));
        }
    }
}
