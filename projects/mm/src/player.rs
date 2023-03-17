use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{AnimationTimer, MainCamera, GAME_SCALE};

pub const PLAYER_ANIM_IDLE: usize = 0;
pub const PLAYER_ANIM_RUN_START: usize = 1;
pub const PLAYER_ANIM_RUN_END: usize = 6;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerState {
    Idle,
    Walking,
    Jumping,
    Falling,
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::Idle,
        }
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

    if is_grounded {
        player.state = PlayerState::Idle;
    }

    let gravity = -1.0 * GAME_SCALE;

    // Process keyboard inputs
    if keyboard_input.pressed(KeyCode::A) {
        velocity.linvel.x = -(12.0 * GAME_SCALE);
    }

    if keyboard_input.pressed(KeyCode::D) {
        velocity.linvel.x = 12.0 * GAME_SCALE;
    }

    if keyboard_input.pressed(KeyCode::Space) && player.state != PlayerState::Jumping {
        velocity.linvel.y += 16.0 * GAME_SCALE;
        player.state = PlayerState::Jumping;
    }

    // Update the translation
    velocity.linvel.x *= 0.75;
    velocity.linvel.y += gravity;

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

// Keeps the camera in sync with the player
pub fn player_camera_system(
    mut main_camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    main_camera_query.single_mut().translation.x = player_query.single().translation.x.round();
    main_camera_query.single_mut().translation.y = player_query.single().translation.y.round();
}
