use crate::resources::{MAP_HEIGHT, MAP_WIDTH};
use amethyst::{
    core::Transform,
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
};

pub fn load_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(MAP_WIDTH * 0.5, MAP_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(MAP_WIDTH, MAP_HEIGHT))
        .with(transform)
        .build();
}
