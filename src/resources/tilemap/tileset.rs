use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PropType {
    Bool(bool),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Property {
    pub name: String,
    pub prop_type: PropType,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Tile {
    pub id: u32,
    pub properties: Vec<Property>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TileSet {
    tiles: Vec<Tile>,
    image: String,
    tile_count: u32,
    spacing: u32,
    tile_height: u32,
    tile_width: u32,
}

