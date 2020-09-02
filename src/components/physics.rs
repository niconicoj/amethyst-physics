use amethyst::ecs::{Component, VecStorage};
use rapier2d::{geometry::ColliderHandle, dynamics::RigidBodyHandle};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Physics {
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Physics {
    pub fn new(body_handle: RigidBodyHandle, collider_handle: ColliderHandle) -> Self {
        Physics {
            body_handle,
            collider_handle,
        }
    }
}
