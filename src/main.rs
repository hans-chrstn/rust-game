use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

const PLAYER_SPEED: f32 = 100.;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, execute_animations).chain())
        .add_systems(
            Update,
            (
                trigger_animation::<CharSprite>.run_if(
                    input_just_pressed(KeyCode::KeyW)
                    .or(input_just_pressed(KeyCode::KeyS)
                    .or(input_just_pressed(KeyCode::KeyA)
                    .or(input_just_pressed(KeyCode::KeyD))))),
            ),
        )
        .run();
}

fn trigger_animation<S: Component>(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AnimationConfig, With<S>>,
) {
    let mut animation = query.single_mut();

    animation.is_running = kb_input.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD]);

    if animation.is_running {
        animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
    }
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
    is_running: bool,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            is_running: false,
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        if config.is_running {
            config.frame_timer.tick(time.delta());

            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.last_sprite_index {
                        atlas.index = config.first_sprite_index;
                    } else {
                        atlas.index += 1;
                    }
                }
            }
        } else {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = config.first_sprite_index;
            }
        }
    }
}

#[derive(Component)]
struct CharSprite;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config = AnimationConfig::new(1, 6, 20);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            }),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(50.0, 0.0, 0.0)),
        CharSprite,
        animation_config,
        Player,
    ));

    commands.spawn((
        Text::new("mwahahahaha panget"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn move_player(
    mut player: Query<(&mut Transform, &mut AnimationConfig), With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
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
        player.scale.x = -player.scale.x.abs();
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
        player.scale.x = player.scale.x.abs();
    }

    if direction != Vec2::ZERO {
        animation.is_running = true;
        let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
        player.translation += move_delta.extend(0.);
    } else {
        animation.is_running = false;
    }
}
