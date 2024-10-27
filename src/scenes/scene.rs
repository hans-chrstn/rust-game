use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use crate::components::sprites::PlayerSprite;

const CAMERA_DECAY_RATE: f32 = 2.;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2)))
    ));

    commands.spawn((
        PlayerSprite,
        Transform::from_xyz(0., 0., 2.)
    ));
}

pub fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<PlayerSprite>)>,
    player: Query<&Transform, (With<PlayerSprite>, Without<Camera2d>)>,
    time: Res<Time>
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

pub fn version_overlay(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("Tree of Life"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        }
    ));
}
