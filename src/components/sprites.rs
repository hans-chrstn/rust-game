use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
    pub is_running: bool
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            is_running: false
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1. / (fps as f32)), TimerMode::Repeating)
    }
}
