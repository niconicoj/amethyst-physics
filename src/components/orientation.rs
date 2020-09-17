use amethyst::ecs::{Component, DenseVecStorage};

pub enum OrientationType {
    Left,
    Right,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Orientation {
    pub value: OrientationType,
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            value: OrientationType::Right
        }
    }
}

