use amethyst::{core::math::Vector2, core::Time, ecs::Entities, ecs::Read, ecs::WriteStorage, ecs::ReadStorage};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, System, SystemData},
};

use crate::components::{BoundingBox, BoundingBoxState, Weight};
#[derive(SystemDesc)]
#[system_desc(name(GravitySystemDesc))]
pub struct GravitySystem {
    gravity: Vector2<f32>,
}

impl Default for GravitySystem {
    fn default() -> Self {
        Self {
            gravity: Vector2::new(0.0, -600.0),
        }
    }
}

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        Entities<'s>, 
        WriteStorage<'s, BoundingBox>,
        Read<'s, Time>,
        ReadStorage<'s, Weight>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities, 
            mut bounding_boxes, 
            time, 
            weights 
        ) = data;

        for (_, bounding_box, _) in (&entities, &mut bounding_boxes, &weights).join() {
            match bounding_box.state {
                BoundingBoxState::Flying => {
                    bounding_box.accelerate(self.gravity * time.delta_seconds());
                    bounding_box.position.y = (bounding_box.position.y as f32).max(50.0);
                    if bounding_box.position.y == 50.0 {
                        bounding_box.state = BoundingBoxState::OnGround;
                    }
                }
                BoundingBoxState::OnGround => {
                    bounding_box.velocity.y = 0.0;
                }
            };
        }
    }
}
