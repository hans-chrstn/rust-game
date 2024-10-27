use bevy::{
    core::FrameCount, prelude::*, window::WindowTheme
};

pub fn toggle_theme(mut window: Single<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyF) {
        if let Some(current_theme) = window.window_theme {
            window.window_theme = match current_theme {
                WindowTheme::Light => Some(WindowTheme::Dark),
                WindowTheme::Dark => Some(WindowTheme::Light)
            };
        }
    }
}

pub fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}
