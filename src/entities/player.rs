use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

use crate::{resources::Context, components::{Physics, Player}};
use rapier2d::{geometry::{ColliderBuilder, ColliderSet}, dynamics::{RigidBodySet, RigidBodyBuilder, BodyStatus}};

pub fn add_player(
    world: &mut World,
    position: (f32, f32),
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let ctx = *world.read_resource::<Context>();
    let body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .translation(position.0, position.1)
        .build();
    let body_handle = {
        let mut body_set = world.write_resource::<RigidBodySet>();
        body_set.insert(body)
    };

    let collider = ColliderBuilder::cuboid(43.*ctx.scale / 2., 64.*ctx.scale / 2.).build();

    let collider_handle = {
        let mut body_set = world.write_resource::<RigidBodySet>();
        let mut collider_set = world.write_resource::<ColliderSet>();
        collider_set.insert(collider, body_handle, &mut body_set)
    };
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(ctx.scale, ctx.scale, ctx.scale));
    transform.set_translation_xyz(position.0, position.1, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2,
    };

    world
        .create_entity()
        .with(Physics::new(body_handle, collider_handle))
        .with(transform)
        .with(Player)
        .with(sprite_render.clone())
        .build();
}
