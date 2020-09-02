use amethyst::prelude::*;

use crate::entities::add_ball;
use crate::resources::SpriteSheetHandle;
use crate::{
    components::{RigidBodyComponent, Ball},
    resources::{MAP_HEIGHT, MAP_WIDTH},
};
use rapier2d::{geometry::ColliderSet, dynamics::RigidBodySet};
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_sheet_handle = {
            let sprite_sheet_list = data.world.read_resource::<SpriteSheetHandle>();
            sprite_sheet_list.get().unwrap().clone()
        };

        data.world.register::<Ball>();
        data.world.register::<RigidBodyComponent>();

        add_ball(
            data.world,
            (MAP_WIDTH / 2., MAP_HEIGHT / 2.),
            sprite_sheet_handle,
        );
    }
}
