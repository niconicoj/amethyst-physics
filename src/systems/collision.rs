use amethyst::{core::Transform, ecs::{System, WriteStorage, ReadStorage, Join}};

use crate::components::BoundingBox;

pub struct BoundingBoxSystem;

impl<'s> System<'s> for BoundingBoxSystem {
    type SystemData = (
        ReadStorage<'s, BoundingBox>,
    );

    fn run(&mut self, data: Self::SystemData) {

    }
}