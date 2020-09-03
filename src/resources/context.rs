use serde::Deserialize;
use ron::de::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Copy, Deserialize)]
pub struct Context {
    pub map_width: f32,
    pub map_height: f32,
    pub scale: f32,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            map_width: 0.,
            map_height: 0.,
            scale: 1.,
        }
    }
}

impl Context {
    pub fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Context, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let context = from_reader(reader)?;
        Ok(context)
    }
}