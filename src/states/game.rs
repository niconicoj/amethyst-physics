use amethyst::prelude::*;

use crate::entities::{add_ball, add_ground, add_player};
use crate::resources::SpriteSheetHandle;
use crate::{
    components::{Physics, Ball, Player, Ground},
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
        data.world.register::<Player>();
        data.world.register::<Ground>();
        data.world.register::<Physics>();

        let ctx = *data.world.read_resource::<Context>();

        add_ball(
            data.world,
            (ctx.map_width * 0.5, ctx.map_height * 0.8),
            sprite_sheet_handle.clone(),
        );
        add_player(
            data.world,
            (ctx.map_width * 0.51, ctx.map_height * 0.5),
            sprite_sheet_handle.clone(),
        );
        add_ground(data.world, 
            (ctx.map_width * 0.5, ctx.map_height * 0.1), 
            ctx.map_width*0.5, 
            sprite_sheet_handle.clone(),
        )
    }
}
