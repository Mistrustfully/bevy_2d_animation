use crate::{components::Animator, AnimationKey};
use bevy::prelude::*;

pub(crate) fn update_animations<T: AnimationKey>(
    mut query: Query<&mut Animator<T>>,
    time: Res<Time>,
) {
    for mut animator in query.iter_mut() {
        if let Some(current_animation) = animator.current_animation {
            let animation = animator
                .animations
                .get_mut(&current_animation)
                .expect("Invalid Animation Key!");

            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                let total_frames = animation.frames.len();
                let current_frame = animator.current_frame;

                let mut next_frame = current_frame + 1;

                if next_frame > total_frames - 1 {
                    next_frame = 0;
                }

                animator.current_frame = next_frame;
            }
        }
    }
}

pub(crate) fn update_spritesheets<T: AnimationKey>(
    mut commands: Commands,
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &Animator<T>,
            Entity,
        ),
        Changed<Animator<T>>,
    >,
) {
    for (mut texture_atlas_sprite, texture_atlas, animator, entity) in query.iter_mut() {
        if let Some(current_animation) = animator.current_animation {
            let animation = animator
                .animations
                .get(&current_animation)
                .expect("Invalid Animation Key!");

            if animation.spritesheet.id() != texture_atlas.id() {
                commands
                    .entity(entity)
                    .insert(animation.spritesheet.clone_weak());
            }

            let frame = animation
                .frames
                .get(animator.current_frame)
                .expect("Invalid Frame!");

            texture_atlas_sprite.index = frame.index;
            texture_atlas_sprite.flip_x = frame.flip_x;
            texture_atlas_sprite.flip_y = frame.flip_y;
        }
    }
}
