use amethyst::{core::Transform, ecs::{System, WriteStorage, ReadStorage, Join}, ecs::Read, core::Time};

use crate::components::BoundingBox;

pub struct TransformSystem;

impl<'s> System<'s> for TransformSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, BoundingBox>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, mut bounding_boxes, time) = data;

        for(transform, bounding_box) in (&mut transforms, &mut bounding_boxes).join() {
            bounding_box.old_position = bounding_box.position;

            bounding_box.position = bounding_box.position + bounding_box.velocity.scale(time.delta_seconds());

            transform.set_translation_x(bounding_box.position.x);
            transform.set_translation_y((bounding_box.position.y as f32).max(0.0));
        }
    }
}