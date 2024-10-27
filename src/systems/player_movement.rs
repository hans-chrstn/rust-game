use bevy::prelude::*;
use crate::components::sprites::{AnimationConfig, PlayerSprite};
use crate::components::player::PLAYER_SPEED;

pub fn move_player(
    mut player: Query<(&mut Transform, &mut AnimationConfig), With<PlayerSprite>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>
) {
    let Ok((mut player, mut animation)) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    
    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
        player.scale.x = player.scale.x.abs();
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
        player.scale.x = player.scale.x.abs();
    }

    if direction != Vec2::ZERO {
        animation.is_running = true;
        let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
        player.translation += move_delta.extend(0.)
    } else {
        animation.is_running = false;
    }
}
