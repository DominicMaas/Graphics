use crate::{
    chunk::{Chunk, ChunkId, CHUNK_XZ},
    terrain::Terrain,
    Player,
};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use super::{ChunkLoadQueue, ChunkLoadTask, RENDER_DISTANCE};

/// Starts the process of managing chunks based on the
///  users view position
pub fn process_chunk_state_on_camera(
    query: Query<&Transform, With<Player>>,
    mut world: ResMut<crate::world::World>,
    mut queue: ResMut<ChunkLoadQueue>,
) {
    let transform = query.single();

    let chunk_x = ((transform.translation.x / CHUNK_XZ as f32) * CHUNK_XZ as f32).floor() as isize;
    let chunk_z = ((transform.translation.z / CHUNK_XZ as f32) * CHUNK_XZ as f32).floor() as isize;

    for x in
        (chunk_x - RENDER_DISTANCE as isize..chunk_x + RENDER_DISTANCE as isize).step_by(CHUNK_XZ)
    {
        for z in (chunk_z - RENDER_DISTANCE as isize..chunk_z + RENDER_DISTANCE as isize)
            .step_by(CHUNK_XZ)
        {
            // Determine the chunk id
            let chunk_id = ChunkId::new(x, z);

            // If this chunk doesn't exist, create it
            if !world.chunks.contains_key(&chunk_id) {
                // Insert a blank chunk into the world
                world.chunks.insert(chunk_id, Chunk::default());

                queue.0.push_back(chunk_id);
            }
        }
    }
}

fn prepare_chunk_load_tasks(
    mut commands: Commands,
    mut queue: ResMut<ChunkLoadQueue>,
    terrain: Res<Terrain>,
    mut world: ResMut<crate::world::World>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    let s = terrain.seed;

    while let Some(chunk_id) = queue.0.pop_front() {
        if let Some(chunk) = world.chunks.get_mut(&chunk_id) {
            let task = thread_pool.spawn(async move {
                Terrain::new(s).generate2(Vec3::new(
                    chunk_id.pos.x as f32,
                    0.0,
                    chunk_id.pos.y as f32,
                ))
            });

            commands.spawn(ChunkLoadTask(task));
        }
    }
}
