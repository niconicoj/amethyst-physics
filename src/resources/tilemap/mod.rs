mod tileset;

use amethyst::{assets::Asset, assets::Handle, assets::ProgressCounter, ecs::VecStorage, ecs::World, ecs::WorldExt};
use serde::{Deserialize, Serialize};

pub use self::tileset::TileSet;

use super::{SpriteSheetList, spritesheet::get_sprite_sheet_handle, AssetType};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Layer {
    pub data: Vec<u32>,
    pub name: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TileMap {
    height: u32,
    width: u32,
    #[serde(rename = "tilewidth")]
    tile_width: u32,
    #[serde(rename = "tileheight")]
    tile_height: u32,
    layers: Vec<Layer>,
    tilesets: Vec<TileSet>,
}

impl Asset for TileMap {
    const NAME: &'static str = "amethyst-physics::TileMap";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<TileMap>>;
}

impl TileMap {
    pub fn register_tileset_spritesheet(
        &self,
        world: &mut World,
        progress_counter: &mut ProgressCounter,
    ) {
        for tileset in &self.tilesets {
            println!("ron path : {}", tileset.properties.ron_path);
            println!("ron path : {}", tileset.properties.texture_path);
            let sprite_sheet_handle = get_sprite_sheet_handle(
                world,
                tileset.properties.texture_path.as_str(),
                tileset.properties.ron_path.as_str(),
                progress_counter,
            );
            let mut sprite_sheet_list = world.write_resource::<SpriteSheetList>();
            sprite_sheet_list.insert(AssetType::TileSet, sprite_sheet_handle);
        }
    }
}
