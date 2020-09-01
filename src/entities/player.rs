use amethyst::{
    assets::Handle,
    core::Transform,
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

use crate::components::Player;

pub fn add_player(
    world: &mut World,
    position: (f32, f32),
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(position.0, position.1, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite_render.clone())
        .build();
}
