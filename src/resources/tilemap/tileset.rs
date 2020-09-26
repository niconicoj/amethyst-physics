use serde::{
    de::{Deserializer, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PropType {
    #[serde(rename = "bool")]
    Bool(bool),
    #[serde(rename = "string")]
    Str(String),
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[serde(deserialize_with = "parse_tileset_property")]
    pub properties: TileSetProps,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct TileSetProps {
    pub texture_path: String,
    pub ron_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", content = "value")]
enum TileSetProperty {
    #[serde(rename = "ron_path")]
    RonPath(String),
    #[serde(rename = "texture_path")]
    TexturePath(String),
}

fn parse_tileset_property<'de, D>(deserializer: D) -> Result<TileSetProps, D::Error>
where
    D: Deserializer<'de>,
{
    struct PropertyParser;
    impl<'de> Visitor<'de> for PropertyParser {
        type Value = TileSetProps;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("[u64, f32, usize]")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut prop = TileSetProps {
                ..Default::default()
            };
            while let Some(tmp) = seq.next_element::<TileSetProperty>()? {
                match tmp {
                    TileSetProperty::RonPath(ron_path) => prop.ron_path = ron_path,
                    TileSetProperty::TexturePath(asset_path) => prop.texture_path = asset_path,
                }
            }
            Ok(prop)
        }
    }
    deserializer.deserialize_any(PropertyParser {})
}
