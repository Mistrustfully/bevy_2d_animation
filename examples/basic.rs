use std::time::Duration;

use bevy::prelude::*;
use bevy_2d_animation::{
    components::{Animator, AnimatorBuilder, RepeatMode},
    AnimationPlayer,
};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Animations {
    A,
    B,
    C,
}

#[derive(Component)]
struct MainSprite {
    current: Animations,
}

fn create_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let size = Vec2::new(16.0, 16.0);

    let texture_handle = asset_server.load("spritesheet.png");
    let texture_handle_alt = asset_server.load("spritesheet-alt.png");

    let texture_atlas = TextureAtlas::from_grid(texture_handle, size, 4, 2, None, None);
    let texture_atlas_alt = TextureAtlas::from_grid(texture_handle_alt, size, 4, 1, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas_handle_alt = texture_atlases.add(texture_atlas_alt);

    let mut animator = AnimatorBuilder::<Animations>::new(
        texture_atlas_handle.clone(),
        Duration::from_secs(1),
        RepeatMode::Loop,
    )
    .register_animation(Animations::A, vec![0, 1, 2, 3])
    .register_animation(Animations::B, vec![4, 5, 6, 7])
    .set_duration(Duration::from_secs(2))
    .set_spritesheet(texture_atlas_handle_alt)
    .register_animation(Animations::C, vec![0, 1, 2, 3])
    .build();

    animator.play_animation(&Animations::A);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                custom_size: Some(size * 10.0),
                ..default()
            },
            ..default()
        },
        MainSprite {
            current: Animations::A,
        },
        animator,
    ));

    commands.spawn(Camera2dBundle::default());
}

fn switch_animation(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Animator<Animations>, &mut MainSprite)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut animator, mut main_sprite) in query.iter_mut() {
            let swapped = match main_sprite.current {
                Animations::A => Animations::B,
                Animations::B => Animations::C,
                Animations::C => Animations::A,
            };

            animator.play_animation(&swapped);
            main_sprite.current = swapped
        }
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AnimationPlayer::<Animations>::new());
    app.add_startup_system(create_sprite)
        .add_system(switch_animation);

    app.run();
}
