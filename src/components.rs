use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::AnimationKey;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum RepeatMode {
    #[default]
    /// Stops playing the animation after the last frame.
    Once,
    /// Loops back to the last frame.
    Loop,
    /// Pauses on last frame.
    Pause,
}

#[derive(Component, Default, Debug)]
pub struct Animator<AnimationKeys: AnimationKey> {
    pub playing: bool,
    pub(crate) animations: HashMap<AnimationKeys, Animation>,
}

impl<AnimationKeys: AnimationKey> Animator<AnimationKeys> {
    pub fn play_animation(&mut self, key: &AnimationKeys) {
        if let Some(animation) = self.animations.get(key) {
            self.stop_animation_by_priority(animation.priority);
        }

        if let Some(animation) = self.animations.get_mut(key) {
            animation.playing = true;
        }
    }

    pub fn restart_animation(&mut self, key: &AnimationKeys) {
        if let Some(animation) = self.animations.get_mut(key) {
            animation.playing = true;
            animation.current_frame = 0;
        }
    }

    pub fn stop_animation_by_key(&mut self, key: &AnimationKeys) {
        if let Some(animation) = self.animations.get_mut(key) {
            animation.playing = false;
        } else {
            warn!("Invalid animation key! {:?}", key);
        }
    }

    pub fn stop_animation_by_priority(&mut self, priority: usize) {
        for (_, animation) in self.animations.iter_mut() {
            if animation.priority == priority {
                animation.playing = false
            }
        }
    }

    pub(crate) fn get_highest_priority_animation(&self) -> Option<&Animation> {
        self.animations
            .iter()
            .max_by(|a, b| {
                let playing_order = a.1.playing.cmp(&b.1.playing);
                if matches!(playing_order, std::cmp::Ordering::Equal) {
                    a.1.priority.cmp(&b.1.priority)
                } else {
                    playing_order
                }
            })
            .map(|(_, v)| v)
    }
}

#[derive(Default, Debug)]
pub struct Animation {
    pub(crate) frames: Vec<Frame>,
    pub(crate) spritesheet: Handle<TextureAtlas>,
    pub(crate) timer: Timer,
    pub(crate) priority: usize,
    pub(crate) current_frame: usize,
    pub(crate) repeat_mode: RepeatMode,
    pub(crate) playing: bool,
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
    priority: usize,
    animations: HashMap<AnimationKeys, Animation>,
    repeat_mode: RepeatMode,
}

impl<AnimationKeys: AnimationKey> AnimatorBuilder<AnimationKeys> {
    pub fn new(
        spritesheet: Handle<TextureAtlas>,
        duration: Duration,
        repeat_mode: RepeatMode,
    ) -> Self {
        Self {
            spritesheet,
            duration,
            priority: 0,
            animations: HashMap::new(),
            repeat_mode,
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

    pub fn set_priority(&mut self, priority: usize) -> &mut Self {
        self.priority = priority;
        self
    }

    pub fn set_repeat_mode(&mut self, repeat_mode: RepeatMode) -> &mut Self {
        self.repeat_mode = repeat_mode;
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
                let cloned = *v;
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
                priority: self.priority,
                current_frame: 0,
                repeat_mode: self.repeat_mode,
                playing: false,
            },
        );
        self
    }

    pub fn build(&mut self) -> Animator<AnimationKeys> {
        Animator {
            playing: false,
            animations: std::mem::take(&mut self.animations),
        }
    }
}
