use crate::components::{BoundingBox, Player, AnimationId, PlayerState};
use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
core::math::Vector2};

pub struct PlayerInputsystem;

impl<'s> System<'s> for PlayerInputsystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, BoundingBox>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut bounding_boxes, input) = data;

        for (player, bounding_box) in (&mut players, &mut bounding_boxes).join() {
            let run_input = input.axis_value("run").expect("run action exists");
            if run_input != 0.0 {
                player.state = PlayerState::Running;
                bounding_box.accelerate(Vector2::new(run_input*5.0, 0.0));
            } else {
                player.state = PlayerState::Idle;
                bounding_box.velocity.x = 0.0 ;
            }
        }
    }
}
