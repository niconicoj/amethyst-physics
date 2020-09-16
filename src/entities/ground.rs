use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

use crate::{components::Ground, resources::Context};

pub fn add_ground(
    world: &mut World,
    position: (f32, f32),
    width: f32,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    // this is messy af. nedd to think of a better way to handle all of this
    let ctx = *world.read_resource::<Context>();

    let mut transform = Transform::default();
    // since wue have to know about the sprite dimension to scale it easily I might want to make a
    transform.set_scale(Vector3::new(1.0 / 128.0 * width, ctx.scale, ctx.scale));
    transform.set_translation_xyz(position.0, position.1, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(transform)
        .with(Ground)
        .with(sprite_render.clone())
        .build();
}
