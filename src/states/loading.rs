use amethyst::{
    assets::{AssetStorage, Handle, JsonFormat, Loader, ProgressCounter},
    ecs::WorldExt,
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans},
    ui::UiCreator,
};

use super::GameState;
use crate::{
    entities::load_camera,
    resources::{load_prefabs, load_sprite_sheets, Context, TileMap},
};

#[derive(Default)]
pub struct LoadingState {
    progress_counter: Option<ProgressCounter>,
    tilemap_handle: Option<Handle<TileMap>>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // context is a resources that general data that everyone might want to know about quickly
        let context = Context::from_config_file("config/context.ron")
            .expect("could not load context from config file.");

        world.insert(context);

        self.progress_counter = Some(load_sprite_sheets(world));
        let mut progress = ProgressCounter::default();
        world.exec(|mut creator: UiCreator<'_>| creator.create("ui/fps.ron", &mut progress));
        load_prefabs(world);

        self.tilemap_handle = {
            let loader = world.read_resource::<Loader>();
            Some(loader.load(
                "maps/map-1.json",
                JsonFormat,
                self.progress_counter.as_mut().expect("map"),
                &world.read_resource::<AssetStorage<TileMap>>(),
            ))
        };

        load_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            if progress_counter.is_complete() {
                self.progress_counter = None;
                return Trans::Switch(Box::new(GameState));
            }
        }
        Trans::None
    }
}
