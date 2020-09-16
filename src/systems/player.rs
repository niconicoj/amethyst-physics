use crate::components::Player;
use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct PlayerInputsystem;

impl<'s> System<'s> for PlayerInputsystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        
    }
}
