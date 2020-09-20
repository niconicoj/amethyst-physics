use amethyst::{
    ecs::{ReadStorage, System},
};

use crate::components::BoundingBox;

pub struct BoundingBoxSystem;

impl<'s> System<'s> for BoundingBoxSystem {
    type SystemData = (ReadStorage<'s, BoundingBox>,);

    fn run(&mut self, _data: Self::SystemData) {}
}
