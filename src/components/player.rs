use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerHealth {
    pub current: i32,
    pub max: i32,
}

pub const PLAYER_SPEED: f32 = 100.;
