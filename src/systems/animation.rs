use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{
    Animation, AnimationId,
PlayerState, Player};

#[derive(Default)]
pub struct AnimationControlSystem;

impl<'s> System<'s> for AnimationControlSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Animation>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
            animations, 
            animation_sets, 
            mut animation_control_sets
        ) = data;

        for (entity, animation, animation_set) in (&entities, &animations, &animation_sets).join() {
            let animation_control_set =
                get_animation_set(&mut animation_control_sets, entity).unwrap();

            if animation.show {
                animation.types.iter().for_each(|&animation_id| {
                    if !animation_control_set.has_animation(animation_id) {
                        let end = match animation_id {
                            AnimationId::Jumping => EndControl::Stay,
                            _ => EndControl::Loop(None),
                        };

                        animation_control_set.add_animation(
                            animation_id,
                            &animation_set.get(&animation_id).unwrap(),
                            end,
                            1.0,
                            AnimationCommand::Init,
                        );
                    }
                });
            }
            animation_control_set.start(animation.current);
        }
    }
}

pub struct PlayerAnimationSystem;

impl<'s> System<'s> for PlayerAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, players, mut animations, mut animation_control_sets) = data;

        for (_entity, player, mut animation, animation_control_set) in (
            &entities,
            &players,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match player.state {
                PlayerState::Idle => AnimationId::Idle,
                PlayerState::Running => AnimationId::Running,
                PlayerState::Jumping => AnimationId::Jumping,
                PlayerState::Falling => AnimationId::Falling,
            };

            // If the new AnimationId is different to the current one, abort the
            // current animation and start the new one
            if animation.current != new_animation_id {
                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);

                animation.current = new_animation_id;
            }
        }
    }
}
