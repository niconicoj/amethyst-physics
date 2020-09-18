use crate::components::{
    BoundingBox, BoundingBoxState, Orientation, OrientationType, Player, PlayerState,
};
use amethyst::{
    core::math::Vector2,
    core::Time,
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct PlayerInputsystem;

impl<'s> System<'s> for PlayerInputsystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, BoundingBox>,
        WriteStorage<'s, Orientation>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut bounding_boxes, mut orientations, input, time) = data;

        for (player, bounding_box, orientation) in
            (&mut players, &mut bounding_boxes, &mut orientations).join()
        {
            let run_input = input.axis_value("run").expect("run action exists");
            let jump_input = input.axis_value("jump").expect("jump action exists");
            match bounding_box.state {
                BoundingBoxState::OnGround => {
                    if run_input != 0.0 {
                        player.state = PlayerState::Running;
                        bounding_box.accelerate(Vector2::new(run_input * 10.0, 0.0));
                        if run_input > 0.0 {
                            orientation.value = OrientationType::Right;
                        } else {
                            orientation.value = OrientationType::Left;
                        }
                    } else {
                        // we should stop in around half a sec, so decelerate at twice or velocity ?
                        player.state = PlayerState::Idle;
                        bounding_box.velocity.x *= 0.8;
                    }
                    if jump_input > 0.0 {
                        bounding_box.accelerate(Vector2::new(0.0, 400.0));
                        player.state = PlayerState::Jumping;
                        bounding_box.state = BoundingBoxState::Flying;
                    }
                }
                BoundingBoxState::Flying => {
                    if run_input != 0.0 {
                        bounding_box.accelerate(Vector2::new(run_input * 2.0, 0.0));
                        if run_input > 0.0 {
                            orientation.value = OrientationType::Right;
                        } else {
                            orientation.value = OrientationType::Left;
                        }
                    }
                    if jump_input <= 0.0 && bounding_box.velocity.y > 0.0 {
                        bounding_box.accelerate(Vector2::new(0.0, -800.0 * time.delta_seconds()));
                    }
                    if bounding_box.velocity.y > 0. {
                        player.state = PlayerState::Jumping;
                    } else {
                        player.state = PlayerState::Falling;
                    }
                }
            }
        }
    }
}
