use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};
use specs_physics::{
    colliders::Shape, nalgebra::Isometry2, nphysics::object::BodyStatus, PhysicsBodyBuilder,
    PhysicsColliderBuilder, SimplePosition,
nalgebra::Vector2};

use crate::{components::Player, resources::Context};

pub fn add_player(
    world: &mut World,
    position: (f32, f32),
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let ctx = *world.read_resource::<Context>();

    let simple_pos = SimplePosition::<f32>(Isometry2::<f32>::translation(position.0, position.1));

    let body = PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
        .gravity_enabled(true)
        .lock_translations(Vector2::new(true, false))
        .build();
    let collider = PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
        half_extents: Vector2::<f32>::new(21.5*ctx.scale, 32.0*ctx.scale),
    })
    .margin(0.2 * ctx.scale)
    .build();

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(ctx.scale, ctx.scale, ctx.scale));
    transform.set_translation_x(position.0);
    transform.set_translation_y(position.1);
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(ctx.scale, ctx.scale, ctx.scale));
    transform.set_translation_xyz(position.0, position.1, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2,
    };

    world
        .create_entity()
        .with(simple_pos)
        .with(transform)
        .with(body)
        .with(collider)
        .with(Player::default())
        .with(sprite_render.clone())
        .build();
}
