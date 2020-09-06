use crate::{components::Ball, resources::Context};
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
};

pub fn add_ball(world: &mut World, position: (f32, f32), sprite_sheet_handle: Handle<SpriteSheet>) {
    let ctx = *world.read_resource::<Context>();

    let simple_pos = SimplePosition::<f32>(Isometry2::<f32>::translation(position.0, position.1));

    let body = PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
        .gravity_enabled(true)
        .build();
    let collider = PhysicsColliderBuilder::<f32>::from(Shape::Ball {
        radius: 32.0 * ctx.scale,
    })
    .margin(0.2 * ctx.scale)
    .build();

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
        .with(simple_pos)
        .with(transform)
        .with(body)
        .with(collider)
        .with(Ball)
        .with(sprite_render.clone())
        .build();
}
