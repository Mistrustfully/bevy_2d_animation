use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::AnimationKey;

#[derive(Component, Default, Debug)]
pub struct Animator<AnimationKeys: AnimationKey> {
    pub playing: bool,
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
        self.playing = true;
    }

    pub fn stop_animation(&mut self) {
        self.current_frame = 0;
        self.playing = false;
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

impl Frame {
    pub fn flip_x(index: usize) -> Self {
        Self {
            index,
            flip_x: true,
            flip_y: false,
        }
    }

    pub fn flip_y(index: usize) -> Self {
        Self {
            index,
            flip_x: false,
            flip_y: true,
        }
    }

    pub fn new(index: usize, flip_x: bool, flip_y: bool) -> Self {
        Self {
            index,
            flip_x,
            flip_y,
        }
    }
}

pub struct AnimatorBuilder<AnimationKeys: AnimationKey> {
    spritesheet: Handle<TextureAtlas>,
    duration: Duration,
    animations: HashMap<AnimationKeys, Animation>,
}

impl<AnimationKeys: AnimationKey> AnimatorBuilder<AnimationKeys> {
    pub fn new(spritesheet: Handle<TextureAtlas>, duration: Duration) -> Self {
        Self {
            spritesheet,
            duration,
            animations: HashMap::new(),
        }
    }

    pub fn set_spritesheet(&mut self, spritesheet: Handle<TextureAtlas>) -> &mut Self {
        self.spritesheet = spritesheet;
        self
    }

    pub fn set_duration(&mut self, duration: Duration) -> &mut Self {
        self.duration = duration;
        self
    }

    pub fn register_animation<T: Into<Frame> + Copy>(
        &mut self,
        key: AnimationKeys,
        frames: Vec<T>,
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
                spritesheet: self.spritesheet.clone(),
                timer: Timer::new(self.duration, TimerMode::Repeating),
            },
        );
        self
    }

    pub fn build(&mut self) -> Animator<AnimationKeys> {
        Animator {
            playing: false,
            animations: std::mem::take(&mut self.animations),
            current_animation: None,
            current_frame: 0,
        }
    }
}
