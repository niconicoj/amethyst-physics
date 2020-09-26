use crate::{
    components::{BoundingBox, Orientation, OrientationType, Player, PlayerState},
    events::PlayerInputEvent,
};
use amethyst::{
    core::math::Vector2,
    core::Time,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

pub struct PlayerKinematicSystemDesc;

impl Default for PlayerKinematicSystemDesc {
    fn default() -> Self {
        PlayerKinematicSystemDesc {}
    }
}

impl<'a, 'b> ::amethyst::core::SystemDesc<'a, 'b, PlayerKinematicSystem>
    for PlayerKinematicSystemDesc
{
    fn build(self, world: &mut ::amethyst::ecs::World) -> PlayerKinematicSystem {
        <PlayerKinematicSystem as ::amethyst::ecs::System<'_>>::SystemData::setup(world);

        let reader_id = world
            .fetch_mut::<EventChannel<PlayerInputEvent>>()
            .register_reader();

        PlayerKinematicSystem::new(reader_id)
    }
}

pub struct PlayerKinematicSystem {
    reader_id: ReaderId<PlayerInputEvent>,
}

impl PlayerKinematicSystem {
    pub fn new(reader_id: ReaderId<PlayerInputEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'s> System<'s> for PlayerKinematicSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, BoundingBox>,
        WriteStorage<'s, Orientation>,
        Read<'s, EventChannel<PlayerInputEvent>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut bounding_boxes, mut orientations, input_event_reader, time) = data;

        for (player, bounding_box, orientation) in
            (&mut players, &mut bounding_boxes, &mut orientations).join()
        {
            for event in input_event_reader.read(&mut self.reader_id) {
                match event {
                    PlayerInputEvent::Jumped => {
                        bounding_box.accelerate(Vector2::new(0.0, 400.0));
                    }
                    PlayerInputEvent::InAirLeft => {
                        bounding_box.accelerate(Vector2::new(-200.0 * time.delta_seconds(), 0.0));
                    }
                    PlayerInputEvent::InAirRight => {
                        bounding_box.accelerate(Vector2::new(200.0 * time.delta_seconds(), 0.0));
                    }
                }
            }
            match player.state {
                PlayerState::Idle => {
                    bounding_box.velocity.x *= 0.8;
                }
                PlayerState::Running => {
                    match orientation.value {
                        OrientationType::Left => {
                            bounding_box
                                .accelerate(Vector2::new(-400.0 * time.delta_seconds(), 0.0));
                        }
                        OrientationType::Right => {
                            bounding_box
                                .accelerate(Vector2::new(400.0 * time.delta_seconds(), 0.0));
                        }
                    };
                }
                PlayerState::Jumping => {}
                PlayerState::Falling => {
                    if bounding_box.velocity.y > 0.0 {
                        bounding_box.accelerate(Vector2::new(0.0, -800.0 * time.delta_seconds()));
                    }
                }
            };
        }
    }
}
