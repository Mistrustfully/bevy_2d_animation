pub mod components;
mod systems;

use bevy::prelude::*;
use core::{fmt::Debug, hash::Hash};
use std::marker::PhantomData;

// Trait Alias
pub trait AnimationKey: Debug + Eq + Hash + Send + Sync + Copy + 'static {}
impl<T> AnimationKey for T where T: Debug + Eq + Hash + Send + Sync + Copy + 'static {}

pub struct AnimationPlayer<AnimationKeys: AnimationKey>(PhantomData<AnimationKeys>);
impl<AnimationKeys: AnimationKey> Plugin for AnimationPlayer<AnimationKeys> {
    fn is_unique(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        "AnimationPlayer"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, systems::update_animations::<AnimationKeys>)
            .add_systems(PostUpdate, systems::update_spritesheets::<AnimationKeys>);
    }
}

impl<AnimationKeys: AnimationKey> Default for AnimationPlayer<AnimationKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
