use bevy::{core_pipeline::bloom::Bloom, prelude::*};

use crate::components::sprites::{AnimationConfig, PlayerSprite};

pub fn player_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = asset_server.load("textures/characters/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config = AnimationConfig::new(1, 6, 10);

    commands.spawn((
        Camera2d,
        Camera {
            ..default()
        },
        Bloom::NATURAL,
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.)).with_translation(Vec3::new(50., 0., 0.)),
        PlayerSprite,
        animation_config
    ));
}
