mod tileset;

use serde::{Deserialize, Serialize};

pub use self::tileset::TileSet;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TileMap {
    height: u32,
    width: u32,
    tile_width: u32,
    tile_height: u32,
    layers: Vec<Layer>,
    tilesets: Vec<TileSet>,
}

