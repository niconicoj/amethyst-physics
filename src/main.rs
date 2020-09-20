use amethyst::{
    animation::AnimationBundle,
    core::{transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::SpriteRender,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
assets::PrefabLoaderSystemDesc, assets::Processor};
use components::{AnimationId, AnimationPrefabData};
use resources::TileMap;

mod components;
mod entities;
mod resources;
mod states;
mod systems;
mod events;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let animation_bundle = AnimationBundle::<AnimationId, SpriteRender>::new(
        "sprite_animation_control",
        "sprite_sampler_interpolation",
    );

    let prefab_loader_system_desc = PrefabLoaderSystemDesc::<AnimationPrefabData>::default();


    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config)?.with_clear([0.05, 0.04, 0.06, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(animation_bundle)?
        .with_bundle(TransformBundle::new().with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(FpsCounterBundle {})?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(rendering_bundle)?
        .with_bundle(systems::PlayerBundle)?
        .with_system_desc(prefab_loader_system_desc, "prefab_loader", &[])
        .with(Processor::<TileMap>::new(), "map_processor", &[])
        .with(systems::GravitySystem::default(), "gravity_system", &[])
        .with(systems::TransformSystem, "transformation_system", &[])
        .with(
            systems::AnimationControlSystem,
            "animation_control_system",
            &[],
        )
        .with(systems::UiFpsSystem::default(), "ui_fps_system", &[])
        .with(systems::OrientationSystem, "orientation_system", &[])
        .with(systems::PlayerAnimationSystem, "player_animation_system", &["animation_control_system"]);

    let mut game =
        Application::build(resources, states::LoadingState::default())?.build(game_data)?;
    game.run();

    Ok(())
}
