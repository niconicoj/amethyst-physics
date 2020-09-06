use amethyst::ecs::{Component, HashMapStorage};

pub enum PlayerStatus {
    Idle,
    Falling,
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Player {
    status: PlayerStatus
}

impl Default for Player {
    fn default() -> Self {
        Player {
            status: PlayerStatus::Idle,
        }
    }
}
