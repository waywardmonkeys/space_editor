pub mod shape;
pub use shape::*;

pub mod material;
pub use material::*;

pub mod camera;
pub use camera::*;

pub mod player_start;
pub use player_start::*;

use bevy::{prelude::*, reflect::*, utils::HashMap};

/// Component to define path to gltf asset that will be loaded after prefab spawn
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct GltfPrefab {
    pub path : String,
    pub scene : String
}

impl Default for GltfPrefab {
    fn default() -> Self {
        Self { 
            scene: "Scene0".to_string(),
            path : String::new()
        }
    }
}

/// Marker for entities spawned from gltf scene
#[derive(Component, Reflect, Default)]
pub struct SceneAutoChild;


/// Not used right now. Planned to be easy method for creating prefab structs from usual structs with assets
#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct AutoStruct<T : Reflect + FromReflect + Default + Clone> {
    pub data : T,
    pub asset_paths : HashMap<String, String>
}

impl<T : Reflect + FromReflect + Default + Clone> AutoStruct<T> {

    pub fn new(data : &T, assets : &AssetServer) -> AutoStruct<T> {
        let mut paths = HashMap::new();

        if let ReflectRef::Struct(s) = data.reflect_ref() {
            for idx in 0..s.field_len() {
                let field_name = s.name_at(idx).unwrap();
                let field = s.field_at(idx).unwrap();
                if let Some(handle) = field.downcast_ref::<Handle<Image>>() {
                    if let Some(path) = assets.get_handle_path(handle) {
                        let path = path.path().to_str().unwrap().to_string();
                        paths.insert(field_name.to_string(), path);
                    }
                }
            }
        }

        AutoStruct::<T> {
            data : data.clone(),
            asset_paths : paths
        }
    }

    pub fn get_data(&self, assets : &AssetServer) -> T {

        let mut res = self.data.clone();
        {
            let mut res_reflect = res.as_reflect_mut();
            if let ReflectMut::Struct(s) = res_reflect.reflect_mut() {

                for (field_name, path) in self.asset_paths.iter() {
                    if let Some(field) = s.field_mut(&field_name) {
                        if let Some(handle) = field.downcast_mut::<Handle<Image>>() {
                            *handle = assets.load(path);
                        } else if let Some(handle) = field.downcast_mut::<Handle<Mesh>>() {
                            *handle = assets.load(path);
                        }
                    }
                }

            }
        }
        T::default()
    }
}


/// This component used in prefab to determine links between entities. Its need to create custom UI in bevy_inspector_egui. You must implement MapEntities trait for your component to make it work. See FollowCamera struct from examples/platformer.rs
#[derive(Reflect, Clone)]
#[reflect(Default)]
pub struct EntityLink {
    pub entity : Entity
}

impl Default for EntityLink {
    fn default() -> Self {
        Self { entity: Entity::PLACEHOLDER }
    }
}
