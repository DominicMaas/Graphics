use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{AnimationTimer, MainCamera};

pub const PLAYER_ANIM_IDLE: usize = 0;
pub const PLAYER_ANIM_RUN_START: usize = 1;
pub const PLAYER_ANIM_RUN_END: usize = 6;

#[derive(Debug)]
pub enum PlayerState {
    Jump { event_time: Duration },
    Land,
    Fall,
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut Player,
        &mut Velocity,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
) {
    let (mut player, mut velocity, mut controller, controller_output) = query.single_mut();

    // Check if the player is grounded
    let is_grounded = if let Some(output) = controller_output {
        output.grounded
    } else {
        false
    };

    let gravity = Vec2::Y * -9.8;

    // Process keyboard inputs
    if keyboard_input.pressed(KeyCode::A) {
        velocity.linvel.x -= 200.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        velocity.linvel.x += 200.0;
    }

    if keyboard_input.pressed(KeyCode::Space) && is_grounded {
        velocity.linvel.y += 350.0;
    }

    // Update the translation
    velocity.linvel.x *= 0.75;
    velocity.linvel.y += gravity.y;

    controller.translation = Some(velocity.linvel * time.delta_seconds());

    // Run for the next frame (slow down player and apply gravity)
}

pub fn player_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Velocity)>,
) {
    let (mut timer, mut sprite, velocity) = query.single_mut();

    timer.tick(time.delta());

    if velocity.linvel.x >= 5.0 || velocity.linvel.x <= -5.0 {
        if timer.just_finished() {
            sprite.index = if sprite.index == PLAYER_ANIM_RUN_END {
                PLAYER_ANIM_RUN_START
            } else {
                sprite.index + 1
            };
        }
    } else {
        sprite.index = PLAYER_ANIM_IDLE;
    }

    sprite.flip_x = velocity.linvel.x <= 0.0;
}
//(&Transform, With<Player>)
pub fn player_camera_system(
    mut main_camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut main_camera_transform = main_camera_query.single_mut();
    let player_transform = player_query.single();

    main_camera_transform.translation.x = player_transform.translation.x.round();
    main_camera_transform.translation.y = player_transform.translation.y.round();
}
