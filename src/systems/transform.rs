
use amethyst::{core::Transform, ecs::{System, WriteStorage, Join, ReadStorage}};
use specs_physics::SimplePosition;

pub struct TransformSystem;

impl<'s> System<'s> for TransformSystem {
    type SystemData = (
        ReadStorage<'s, SimplePosition<f32>>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut transforms) = data;

        for (position, transform) in (&positions, &mut transforms).join() {
            transform.set_translation_x(position.0.translation.vector.x); 
            transform.set_translation_y(position.0.translation.vector.y); 
        }
    }
}