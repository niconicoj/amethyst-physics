use amethyst::prelude::*;

use crate::{entities::{add_ball, add_ground, add_player}, resources::PrefabList, resources::PrefabType, components::BoundingBox};
use crate::resources::SpriteSheetHandle;
use crate::{
    components::{Ball, Ground, Player},
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
        data.world.register::<Ground>();
        data.world.register::<Player>();
        data.world.register::<BoundingBox>();

        let ctx = *data.world.read_resource::<Context>();

        add_ball(
            data.world,
            (ctx.map_width * 0.5, ctx.map_height * 0.8),
            sprite_sheet_handle.clone(),
        );

        let prefab_handle = {
            let prefab_list = data.world.read_resource::<PrefabList>();
            prefab_list.get(PrefabType::Player).unwrap().clone()
        };

        add_player(
            data.world,
            (ctx.map_width * 0.6, ctx.map_height * 0.9),
            prefab_handle
        );
        add_ground(data.world, 
            (ctx.map_width * 0.5, ctx.map_height * 0.1), 
            ctx.map_width*0.5, 
            sprite_sheet_handle.clone(),
        );
    }
}
