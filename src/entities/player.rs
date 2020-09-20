use amethyst::{
    assets::Handle,
    assets::Prefab,
    core::math::Vector2,
    core::{math::Vector3, Transform},
    prelude::{Builder, WorldExt},
    shred::World,
};

use crate::{
    components::Animation, components::AnimationId, components::AnimationPrefabData,
    components::BoundingBox, components::Orientation, components::Player, resources::Context,
components::Weight};

pub fn add_player(
    world: &mut World,
    position: (f32, f32),
    prefab: Handle<Prefab<AnimationPrefabData>>,
) {
    let ctx = *world.read_resource::<Context>();

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(
        ctx.scale * 2.0,
        ctx.scale * 2.0,
        ctx.scale * 2.0,
    ));
    transform.set_translation_x(position.0);
    transform.set_translation_y(position.1);

    let bounding_box = BoundingBox::new(
        Vector2::new(position.0, position.1),
        Vector2::new(21.5 * ctx.scale, 32.0 * ctx.scale),
        Vector2::new(200.0, 400.0),
    );

    world
        .create_entity()
        .with(transform)
        .with(bounding_box)
        .with(Player::default())
        .with(prefab)
        .with(Orientation::default())
        .with(Weight)
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Idle,
                AnimationId::Running,
                AnimationId::Jumping,
                AnimationId::Falling,
            ],
        ))
        .build();
}
