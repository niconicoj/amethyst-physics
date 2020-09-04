use crate::components::Physics;
use crate::components::Player;
use amethyst::{
    ecs::{Join, Read, ReadStorage, WriteExpect},
    input::{InputHandler, StringBindings},
    shred::System,
};
use rapier2d::{dynamics::RigidBodySet, geometry::ColliderSet, na::Vector2};
pub struct PlayerInputSystem;

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        WriteExpect<'a, RigidBodySet>,
        WriteExpect<'a, ColliderSet>,
        ReadStorage<'a, Physics>,
        ReadStorage<'a, Player>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut bodies_set, mut _colliders_set, physics_handles, player_flags, input) = data;

        for (physics_handle, _ ) in (&physics_handles, &player_flags).join() {
            let run_input = input.axis_value("move_x").expect("Move x action exists");
            // maybe we should not panic here ?
            let mut body = bodies_set
                .get_mut(physics_handle.body_handle)
                .expect("could not find a body in set for entity");

            body.apply_force(Vector2::new(run_input * 4., 0.));
        }
    }
}
