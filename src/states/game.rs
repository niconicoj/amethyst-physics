use amethyst::prelude::*;

use crate::entities::add_player;
use crate::resources::SpriteSheetHandle;
use crate::{
    components::Player,
    resources::{MAP_HEIGHT, MAP_WIDTH},
};
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_sheet_handle = {
            let sprite_sheet_list = data.world.read_resource::<SpriteSheetHandle>();
            sprite_sheet_list.get().unwrap().clone()
        };
        data.world.register::<Player>();
        add_player(
            data.world,
            (MAP_WIDTH / 2., MAP_HEIGHT / 2.),
            sprite_sheet_handle,
        );
    }
}
