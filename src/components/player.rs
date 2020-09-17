use amethyst::ecs::{Component, HashMapStorage};

pub enum PlayerState {
    Idle,
    Jumping,
    Falling,
    Running,
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Player {
    pub state: PlayerState,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            state: PlayerState::Idle,
        }
    }
}
