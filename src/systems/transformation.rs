use crate::components::Physics;
use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, WriteStorage},
    shred::{ReadExpect, System},
};
use rapier2d::{dynamics::RigidBodySet, geometry::ColliderSet};

pub struct TransformationSystem;

impl<'a> System<'a> for TransformationSystem {
    type SystemData = (
        ReadStorage<'a, Physics>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, RigidBodySet>,
        ReadExpect<'a, ColliderSet>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (bodies, mut transforms, bodies_set, _colliders_set) = data;

        for (body_handle, transform) in (&bodies, &mut transforms).join() {
            // maybe we should not panic here ?
            let body = bodies_set.get(body_handle.body_handle).expect("could not find a body in set for entity");
            transform.set_translation_x(body.position.translation.vector.x);
            transform.set_translation_y(body.position.translation.vector.y);
        }
    }
}
