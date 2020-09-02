use amethyst::prelude::*;

use crate::entities::add_ball;
use crate::resources::SpriteSheetHandle;
use crate::{
    components::{Physics, Ball},
    resources::Context,
};
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_sheet_handle = {
            let sprite_sheet_list = data.world.read_resource::<SpriteSheetHandle>();
            sprite_sheet_list.get().unwrap().clone()
        };

        data.world.register::<Ball>();
        data.world.register::<Physics>();

        let ctx = *data.world.read_resource::<Context>();

        add_ball(
            data.world,
            (ctx.map_width / 2., ctx.map_height / 2.),
            sprite_sheet_handle,
        );
    }
}
