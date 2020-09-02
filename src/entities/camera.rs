use crate::resources::Context;
use amethyst::{
    core::Transform,
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
};

pub fn load_camera(world: &mut World) {
    let mut transform = Transform::default();
    let ctx = *world.read_resource::<Context>();
    transform.set_translation_xyz(ctx.map_width * 0.5, ctx.map_height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ctx.map_width, ctx.map_height))
        .with(transform)
        .build();
}
