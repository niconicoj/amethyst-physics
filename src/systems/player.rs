use crate::components::Player;
use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};
use specs_physics::{nalgebra::Vector2, nphysics::algebra::{Velocity2, Force2}, PhysicsBody};

pub struct PlayerInputsystem;

impl<'s> System<'s> for PlayerInputsystem {
    type SystemData = (
        WriteStorage<'s, PhysicsBody<f32>>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut bodies, player, input) = data;

        for (body, _) in (&mut bodies, &player).join() {
            let run_input = input.axis_value("run").expect("run action exists");
            body.velocity += Velocity2::<f32>::new(Vector2::<f32>::new(run_input*0.1, 0.0), 0.0);
        }
    }
}
