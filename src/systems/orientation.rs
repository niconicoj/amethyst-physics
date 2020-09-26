
use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use std::f32::consts::PI;

use crate::components::{Orientation, OrientationType};

pub struct OrientationSystem;

impl<'s> System<'s> for OrientationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Orientation>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, orientations, mut transforms): Self::SystemData) {
        // Iterate over entities having direction and transform components
        for (_, orientation, transform) in (&entities, &orientations, &mut transforms).join() {
            match orientation.value {
                OrientationType::Left => transform.set_rotation_y_axis(PI),
                OrientationType::Right => transform.set_rotation_y_axis(0.0),
            };
        }
    }
}