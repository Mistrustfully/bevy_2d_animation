use crate::{components::Animator, AnimationKey};
use bevy::prelude::*;

pub(crate) fn update_animations<T: AnimationKey>(
    mut query: Query<&mut Animator<T>>,
    time: Res<Time>,
) {
    for mut animator in query.iter_mut() {
        if !animator.playing {
            continue;
        };

        if let Some(current_animation) = animator.current_animation {
            if let Some(animation) = animator.animations.get_mut(&current_animation) {
                animation.timer.tick(time.delta());
                if animation.timer.just_finished() {
                    let total_frames = animation.frames.len();
                    let current_frame = animator.current_frame;

                    let next_frame = current_frame + 1;

                    if next_frame > total_frames - 1 {
                        animator.stop_animation();
                        return;
                    }

                    animator.current_frame = next_frame;
                }
            } else {
                warn!("Invalid Animation Key!, {:?}", current_animation);
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
        if let Some(current_animation) = animator.current_animation {
            if let Some(animation) = animator.animations.get(&current_animation) {
                if animation.spritesheet.id() != texture_atlas.id() {
                    *texture_atlas = animation.spritesheet.clone();
                }

                let frame = animation
                    .frames
                    .get(animator.current_frame)
                    .expect("Invalid Frame!");

                texture_atlas_sprite.index = frame.index;
                texture_atlas_sprite.flip_x = frame.flip_x;
                texture_atlas_sprite.flip_y = frame.flip_y;
            } else {
                warn!("Invalid Animation Key: {:?}", current_animation);
            }
        }
    }
}
