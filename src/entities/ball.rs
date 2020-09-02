use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

use crate::{resources::Context, components::{Physics, Ball}};
use rapier2d::{geometry::{ColliderBuilder, ColliderSet}, dynamics::{RigidBodySet, RigidBodyBuilder, BodyStatus}};

pub fn add_ball(
    world: &mut World,
    position: (f32, f32),
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .translation(position.0, position.1)
        .build();
    let body_handle = {
        let mut body_set = world.write_resource::<RigidBodySet>();
        body_set.insert(body)
    };

    let collider = ColliderBuilder::ball(32.0).build();

    let collider_handle = {
        let mut body_set = world.write_resource::<RigidBodySet>();
        let mut collider_set = world.write_resource::<ColliderSet>();
        collider_set.insert(collider, body_handle, &mut body_set)
    };
    let ctx = *world.read_resource::<Context>();
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(ctx.scale, ctx.scale, ctx.scale));
    transform.set_translation_xyz(position.0, position.1, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Physics::new(body_handle, collider_handle))
        .with(transform)
        .with(Ball)
        .with(sprite_render.clone())
        .build();
}