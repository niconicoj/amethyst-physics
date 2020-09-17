use amethyst::core::math::Vector2;
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct BoundingBox {
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub half_size: Vector2<f32>,
    pub max_velocity: Vector2<f32>,
    pub on_ground: bool,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            old_position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            half_size: Vector2::new(0.0, 0.0),
            max_velocity: Vector2::new(0.0, 0.0),
            on_ground: false,
        }
    }
}

impl BoundingBox {
    pub fn new(
        position: Vector2<f32>,
        half_size: Vector2<f32>,
        max_velocity: Vector2<f32>,
    ) -> Self {
        Self {
            position,
            old_position: position,
            velocity: Vector2::new(0.0, 0.0),
            half_size,
            max_velocity,
            on_ground: false,
        }
    }

    pub fn accelerate(&mut self, acceleration: Vector2<f32>) {
        self.velocity += acceleration;
        self.velocity.x = (self.velocity.x as f32)
            .min(self.max_velocity.x.abs())
            .max(-self.max_velocity.x.abs());
        self.velocity.y = (self.velocity.y as f32)
            .min(self.max_velocity.y.abs())
            .max(-self.max_velocity.y.abs());
    }

    pub fn overlaps(&self, other: BoundingBox) -> bool {
        !((self.position.x - other.position.x).abs() > self.half_size.x + other.half_size.x
            || (self.position.y - other.position.y).abs() > self.half_size.y + other.half_size.y)
    }
}
