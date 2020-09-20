use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PropType {
    #[serde(rename = "bool")]
    Bool(bool),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Property {
    pub name: String,
    #[serde(flatten)]
    pub prop_type: PropType,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Tile {
    pub id: u32,
    pub properties: Vec<Property>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TileSet {
    pub tiles: Vec<Tile>,
    #[serde(rename = "image")]
    pub path: String,
    #[serde(rename = "tilecount")]
    pub tile_count: u32,
    pub spacing: u32,
    #[serde(rename = "tileheight")]
    pub tile_height: u32,
    #[serde(rename = "tilewidth")]
    pub tile_width: u32,
}

