use amethyst::{core::Transform, ecs::{Component, VecStorage}};
use rapier2d::{geometry::ColliderHandle, dynamics::RigidBodyHandle};

#[derive(Component)]
#[storage(VecStorage)]
pub struct RigidBodyComponent {
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl RigidBodyComponent {
    pub fn new(body_handle: RigidBodyHandle, collider_handle: ColliderHandle) -> Self {
        RigidBodyComponent {
            body_handle,
            collider_handle,
        }
    }
}
