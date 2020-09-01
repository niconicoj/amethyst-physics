use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    ecs::prelude::World,
    prelude::WorldExt,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        Texture,
    },
};

const TEXTURE_PATH: &str = "spritesheet.png";
const RON_PATH: &str = "spritesheet.ron";

#[derive(Default)]
pub struct SpriteSheetHandle {
    spritesheet: Option<Handle<SpriteSheet>>,
}

impl SpriteSheetHandle {
    pub fn insert(&mut self, handle: Handle<SpriteSheet>) {
        self.spritesheet = Some(handle);
    }

    pub fn get(&self) -> Option<&Handle<SpriteSheet>> {
        self.spritesheet.as_ref()
    }
}

pub fn load_sprite_sheet(world: &mut World) -> ProgressCounter {
    let mut sprite_sheet_entity = SpriteSheetHandle::default();
    let mut progress_counter = ProgressCounter::new();

    let sprite_sheet_handle =
        get_sprite_sheet_handle(world, TEXTURE_PATH, RON_PATH, &mut progress_counter);
    sprite_sheet_entity.insert(sprite_sheet_handle);
    world.insert(sprite_sheet_entity);
    progress_counter
}

fn get_sprite_sheet_handle(
    world: &mut World,
    texture_path: &str,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sprite_sheet_store,
    )
}
