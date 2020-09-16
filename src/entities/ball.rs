use crate::{components::Ball, resources::Context};
use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

pub fn add_ball(world: &mut World, position: (f32, f32), sprite_sheet_handle: Handle<SpriteSheet>) {
    let ctx = *world.read_resource::<Context>();

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(ctx.scale, ctx.scale, ctx.scale));
    transform.set_translation_x(position.0);
    transform.set_translation_y(position.1);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(Ball)
        .with(sprite_render.clone())
        .build();
}
