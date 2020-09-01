use amethyst::{
    assets::ProgressCounter,
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans},
    ui::UiCreator,
};

use super::GameState;
use crate::{entities::load_camera, resources::load_sprite_sheet};

#[derive(Default)]
pub struct LoadingState {
    progress_counter: Option<ProgressCounter>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.progress_counter = Some(load_sprite_sheet(world));
        let mut progress = ProgressCounter::default();
        world.exec(|mut creator: UiCreator<'_>| creator.create("ui/fps.ron", &mut progress));
        load_camera(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            if progress_counter.is_complete() {
                self.progress_counter = None;
                return Trans::Switch(Box::new(GameState));
            }
        }
        Trans::None
    }
}
