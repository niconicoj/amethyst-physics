mod tileset;

use amethyst::{ecs::VecStorage, assets::Asset, assets::Handle, ecs::World};
use serde::{Deserialize, Serialize};

pub use self::tileset::TileSet;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Layer {
    pub data: Vec<u32>,
    pub name: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub fn register_tileset_spritesheet(&self, world: &mut World) {
        for tileset in &self.tilesets {
            println!("tileset path : {:?}", tileset.path);
        }
    }
}

