use amethyst::{
    core::{
        bundle::SystemBundle,
        SystemDesc,
    },
    ecs::{DispatcherBuilder, World},
};

use amethyst::Error;
use core::result::Result;

mod inputs;
mod kinematics;

#[derive(Debug)]
pub struct PlayerBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PlayerBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(inputs::PlayerInputsystem, "player_input_system", &[]);
        builder.add(
            kinematics::PlayerKinematicSystemDesc::default().build(world),
            "player_kinematic_system",
            &["player_input_system"],
        );
        Ok(())
    }
}