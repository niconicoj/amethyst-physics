use amethyst::ecs::{Component, NullStorage};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ground;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ball;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Weight;
