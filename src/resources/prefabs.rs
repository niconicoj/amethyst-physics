use amethyst::{
    assets::{ProgressCounter, Handle, Prefab, PrefabLoader, RonFormat},
    ecs::prelude::World,
};
use strum_macros::EnumString;
use std::slice::Iter;
use std::collections::HashMap;

use crate::components::AnimationPrefabData;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, EnumString)]
pub enum PrefabType {
    Player,
}

impl PrefabType {
    pub fn into_iter() -> Iter<'static, PrefabType> {
        static PREFAB_TYPE: [PrefabType; 1] = [
            PrefabType::Player, 
            ];
        PREFAB_TYPE.iter()
    }
}

#[derive(Default)]
pub struct PrefabList {
    prefabs: HashMap<PrefabType, Handle<Prefab<AnimationPrefabData>>>,
}

impl PrefabList {
    pub fn insert(
        &mut self,
        asset_type: PrefabType,
        prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    ) {
        self.prefabs.insert(asset_type, prefab_handle);
    }

    pub fn get(&self, asset_type: PrefabType) -> Option<&Handle<Prefab<AnimationPrefabData>>> {
        self.prefabs.get(&asset_type)
    }
}

pub fn load_prefabs(world: &mut World) -> ProgressCounter {
    let mut prefab_list = PrefabList::default();
    let mut progress_counter = ProgressCounter::new();

    for &prefab_type in PrefabType::into_iter() {
        println!("loading prefab : {:?}", prefab_type);
        let (_texture_path, ron_path) = match prefab_type {
            PrefabType::Player => (
                "textures/player.png", 
                "prefabs/player.ron"
            ),
        };
        let prefab_handle =
            get_animation_prefab_handle(world, ron_path, &mut progress_counter);
            prefab_list.insert(prefab_type, prefab_handle);
    }
    world.insert(prefab_list);
    progress_counter
}

fn get_animation_prefab_handle(
    world: &mut World,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<AnimationPrefabData>> {
    world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load(ron_path, RonFormat, progress_counter)
    })
}

