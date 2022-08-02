use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::chunk::Chunk;
use crate::chunk::CHUNK_SIZE;
use crate::pixel::Pixel;
use crate::MainCamera;
use crate::WorldState;

/// The input system handles translating user mouse clicks into
/// game events
pub fn input_system(
    windows: Res<Windows>,
    mut world_state: ResMut<WorldState>,
    buttons: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_chunk: Query<&mut Chunk>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        let adj_pos = world_pos + Vec2::new(CHUNK_SIZE as f32 / 2.0, CHUNK_SIZE as f32 / 2.0);

        if buttons.pressed(MouseButton::Left) {
            for mut c in q_chunk.iter_mut() {
                if world_state.brush_size == 1 {
                    c.overwrite_pixel(adj_pos, world_state.brush_type);
                } else {
                    for x in -world_state.brush_size..world_state.brush_size {
                        for y in -world_state.brush_size..world_state.brush_size {
                            if x * x + y * y <= world_state.brush_size * world_state.brush_size {
                                c.overwrite_pixel(
                                    adj_pos + Vec2::new(x as f32, y as f32),
                                    world_state.brush_type,
                                );
                            }
                        }
                    }
                }

                //c.set_pixel(adj_pos, Pixel::new(world_state.brush_type));
            }
        }
    }
}
