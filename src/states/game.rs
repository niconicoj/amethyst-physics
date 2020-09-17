use amethyst::prelude::*;

use crate::{entities::add_player, resources::PrefabList, resources::PrefabType, components::BoundingBox};
use crate::{
    components::Player,
    resources::Context,
};
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.register::<Player>();
        data.world.register::<BoundingBox>();

        let ctx = *data.world.read_resource::<Context>();

        let prefab_handle = {
            let prefab_list = data.world.read_resource::<PrefabList>();
            prefab_list.get(PrefabType::Player).unwrap().clone()
        };

        add_player(
            data.world,
            (ctx.map_width * 0.6, ctx.map_height * 0.9),
            prefab_handle
        );
    }
}
