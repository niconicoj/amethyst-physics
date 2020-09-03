use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

use crate::{resources::Context, components::{Physics, Ground}};
use rapier2d::{geometry::{ColliderBuilder, ColliderSet}, dynamics::{RigidBodySet, RigidBodyBuilder, BodyStatus}};

pub fn add_ground(
    world: &mut World,
    position: (f32, f32),
    width: f32,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let ctx = *world.read_resource::<Context>();

    let body = RigidBodyBuilder::new(BodyStatus::Static)
        .translation(position.0, position.1)
        .build();
    let body_handle = {
        let mut body_set = world.write_resource::<RigidBodySet>();
        body_set.insert(body)
    };

    let collider = ColliderBuilder::cuboid(width/2.0, 32.0*ctx.scale).build();

    let collider_handle = {
        let mut body_set = world.write_resource::<RigidBodySet>();
        let mut collider_set = world.write_resource::<ColliderSet>();
        collider_set.insert(collider, body_handle, &mut body_set)
    };
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
        .with(Physics::new(body_handle, collider_handle))
        .with(transform)
        .with(Ground)
        .with(sprite_render.clone())
        .build();
}