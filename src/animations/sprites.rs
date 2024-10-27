use bevy::prelude::*;
use crate::components::sprites::AnimationConfig;

pub fn execute_animations(time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>
) {
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
