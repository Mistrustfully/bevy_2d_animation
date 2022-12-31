use crate::{
    components::{Animator, RepeatMode},
    AnimationKey,
};
use bevy::prelude::*;

pub(crate) fn update_animations<T: AnimationKey>(
    mut query: Query<&mut Animator<T>>,
    time: Res<Time>,
) {
    for mut animator in query.iter_mut() {
        if let Some(key) = animator.get_highest_priority_key() {
            if let Some(animation) = animator.animations.get_mut(&key) {
                if !animation.playing {
                    return;
                };

                animation.timer.tick(time.delta());
                if animation.timer.just_finished() {
                    let total_frames = animation.frames.len();
                    let current_frame = animation.current_frame;

                    let next_frame = current_frame + 1;

                    if next_frame > total_frames - 1 {
                        return match animation.repeat_mode {
                            RepeatMode::Loop => {
                                animation.current_frame = 0;
                            }
                            RepeatMode::Once => {
                                animation.current_frame = 0;
                                animator.stop_animation_by_key(key);
                            }
                            RepeatMode::Pause => {
                                animation.playing = false;
                            }
                        };
                    }

                    animation.current_frame = next_frame;
                }
            } else {
                warn!("Invalid Animation Key!, {:?}", key);
            }
        }
    }
}

pub(crate) fn update_spritesheets<T: AnimationKey>(
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
            &Animator<T>,
        ),
        Changed<Animator<T>>,
    >,
) {
    for (mut texture_atlas_sprite, mut texture_atlas, animator) in query.iter_mut() {
        if let Some(key) = animator.get_highest_priority_key() {
            if let Some(animation) = animator.animations.get(&key) {
                if animation.spritesheet.id() != texture_atlas.id() {
                    *texture_atlas = animation.spritesheet.clone();
                }

                let frame = animation
                    .frames
                    .get(animation.current_frame)
                    .expect("Invalid Frame!");

                texture_atlas_sprite.index = frame.index;
                texture_atlas_sprite.flip_x = frame.flip_x;
                texture_atlas_sprite.flip_y = frame.flip_y;
            } else {
                warn!("Invalid Animation Key: {:?}", key);
            }
        }
    }
}
