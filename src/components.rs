use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::AnimationKey;

#[derive(Component, Default, Debug)]
pub struct Animator<AnimationKeys: AnimationKey> {
    pub(crate) animations: HashMap<AnimationKeys, Animation>,
    pub(crate) current_animation: Option<AnimationKeys>,
    pub(crate) current_frame: usize,
}

impl<AnimationKeys: AnimationKey> Animator<AnimationKeys> {
    pub fn play_animation(&mut self, key: AnimationKeys) {
        if let Some(current_animation) = self.current_animation {
            if current_animation != key {
                self.current_frame = 0;
            }
        }
        self.current_animation = Some(key);
    }

    pub fn stop_animation(&mut self) {
        self.current_animation = None;
        self.current_frame = 0;
    }
}

#[derive(Default, Debug)]
pub struct Animation {
    pub(crate) frames: Vec<Frame>,
    pub(crate) spritesheet: Handle<TextureAtlas>,
    pub(crate) timer: Timer,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Frame {
    pub index: usize,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl From<usize> for Frame {
    fn from(value: usize) -> Self {
        Self {
            index: value,
            flip_x: false,
            flip_y: false,
        }
    }
}

pub struct AnimatorBuilder<AnimationKeys: AnimationKey> {
    spritesheet: Handle<TextureAtlas>,
    animations: HashMap<AnimationKeys, Animation>,
}

impl<AnimationKeys: AnimationKey> AnimatorBuilder<AnimationKeys> {
    pub fn new(spritesheet: Handle<TextureAtlas>) -> Self {
        Self {
            spritesheet,
            animations: HashMap::new(),
        }
    }

    pub fn set_spritesheet(&mut self, spritesheet: Handle<TextureAtlas>) -> &mut Self {
        self.spritesheet = spritesheet;
        self
    }

    pub fn register_animation<T: Into<Frame> + Copy>(
        &mut self,
        key: AnimationKeys,
        frames: Vec<T>,
        duration: Duration,
    ) -> &mut Self {
        let converted_frames = frames
            .iter()
            .map(|v| {
                let cloned = v.clone();
                let frame: Frame = cloned.into();
                frame
            })
            .collect();

        self.animations.insert(
            key,
            Animation {
                frames: converted_frames,
                spritesheet: self.spritesheet.clone_weak(),
                timer: Timer::new(duration, TimerMode::Repeating),
            },
        );
        self
    }

    pub fn build(&mut self) -> Animator<AnimationKeys> {
        Animator {
            animations: std::mem::take(&mut self.animations),
            current_animation: None,
            current_frame: 0,
        }
    }
}
