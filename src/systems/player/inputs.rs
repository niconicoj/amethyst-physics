use crate::{
    components::{
        BoundingBox, BoundingBoxState, Orientation, OrientationType, Player, PlayerState,
    },
    events::PlayerInputEvent,
};
use amethyst::{
    ecs::Write,
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

pub struct PlayerInputsystem;

impl<'s> System<'s> for PlayerInputsystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, BoundingBox>,
        WriteStorage<'s, Orientation>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EventChannel<PlayerInputEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut bounding_boxes, mut orientations, input, mut player_input_channel) =
            data;

        for (player, bounding_box, orientation) in
            (&mut players, &mut bounding_boxes, &mut orientations).join()
        {
            let run_input = input.axis_value("run").expect("run axis exists");
            let jump_action = input.action_is_down("jump").expect("jump action exists");

            match bounding_box.state {
                BoundingBoxState::OnGround => {
                    if run_input != 0.0 {
                        player.state = PlayerState::Running;
                        if run_input > 0.0 {
                            orientation.value = OrientationType::Right;
                        } else {
                            orientation.value = OrientationType::Left;
                        }
                    } else {
                        player.state = PlayerState::Idle;
                    }
                    if jump_action {
                        player.state = PlayerState::Jumping;
                        bounding_box.state = BoundingBoxState::Flying;
                        player_input_channel.single_write(PlayerInputEvent::Jumped);
                    }
                }
                BoundingBoxState::Flying => {
                    if run_input != 0.0 {
                        if run_input > 0.0 {
                            orientation.value = OrientationType::Right;
                            player_input_channel.single_write(PlayerInputEvent::InAirRight);
                        } else {
                            orientation.value = OrientationType::Left;
                            player_input_channel.single_write(PlayerInputEvent::InAirLeft);
                        }
                    }
                    if jump_action && bounding_box.velocity.y > 0.0 {
                        player.state = PlayerState::Jumping;
                    } else {
                        player.state = PlayerState::Falling;
                    }
                }
            }
        }
    }
}
