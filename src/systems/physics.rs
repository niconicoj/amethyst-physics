use amethyst::{core::math::Vector2, ecs::WriteStorage, ecs::Entities, ecs::Read, core::Time};
use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Join},
};

use crate::components::BoundingBox;
#[derive(SystemDesc)]
#[system_desc(name(GravitySystemDesc))]
pub struct GravitySystem {
    gravity: Vector2<f32>,
}

impl GravitySystem {
    fn new(gravity: Vector2<f32>) -> Self {
        Self {
            gravity,
        }
    }
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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut bounding_boxes, time) = data;

        for (_, bounding_box) in (&entities, &mut bounding_boxes).join() {
            if !bounding_box.on_ground {
                bounding_box.accelerate(self.gravity*time.delta_seconds());
                bounding_box.position.y = (bounding_box.position.y as f32).max(50.0);
                if bounding_box.position.y == 50.0 {
                    bounding_box.on_ground = true;
                }  
            } else {
                bounding_box.velocity.y = 0.0;
            }
            
        }
    }
}