use amethyst::ecs::{Component, HashMapStorage};

pub enum PlayerState {
    Idle,
    Jumping,
    Running,
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Player {
    pub state: PlayerState,
    on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            state: PlayerState::Idle,
            on_ground: true,
        }
    }
}
