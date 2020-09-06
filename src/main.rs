use amethyst::{
    core::{transform::TransformBundle, Transform},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    
};
use specs_physics::{
    systems::{
        PhysicsStepperSystem,
        SyncBodiesFromPhysicsSystem,
        SyncBodiesToPhysicsSystem,
        SyncCollidersToPhysicsSystem,
        SyncParametersToPhysicsSystem,
    },
    SimplePosition,
};

mod components;
mod entities;
mod resources;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(FpsCounterBundle {})?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(
            SyncBodiesToPhysicsSystem::<f32, SimplePosition<f32>>::default(),
            "sync_bodies_to_physics_system",
            &[],
        )
        .with(
            SyncCollidersToPhysicsSystem::<f32, SimplePosition<f32>>::default(),
            "sync_colliders_to_physics_system",
            &["sync_bodies_to_physics_system"],
        )
        .with(
            SyncParametersToPhysicsSystem::<f32>::default(),
            "sync_gravity_to_physics_system",
            &[],
        )
        .with(
            PhysicsStepperSystem::<f32>::default(),
            "physics_stepper_system",
            &[
                "sync_bodies_to_physics_system",
                "sync_colliders_to_physics_system",
                "sync_gravity_to_physics_system",
            ],
        )
        .with(
            SyncBodiesFromPhysicsSystem::<f32, SimplePosition<f32>>::default(),
            "sync_bodies_from_physics_system",
            &["physics_stepper_system"],
        )
        .with(
            systems::TransformSystem,
            "transformation_system",
            &["sync_bodies_from_physics_system"]
        )
        .with(systems::UiFpsSystem::default(), "ui_fps_system", &[])
        .with(systems::PlayerInputsystem, "player_input_system", &["sync_bodies_from_physics_system"]);

    let mut game = Application::build(resources, states::LoadingState::default())?
        .build(game_data)?;
    game.run();

    Ok(())
}
