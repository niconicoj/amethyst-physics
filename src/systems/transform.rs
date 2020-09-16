
use amethyst::{core::Transform, ecs::{System, WriteStorage, Join, ReadStorage}};

pub struct TransformSystem;

impl<'s> System<'s> for TransformSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
    
    }
}