use bevy::{ecs::system::EntityCommands, utils::HashMap};

use crate::{ext::*, PrefabMarker, prefab::component::*};

pub const MESH_CATEGORY : &str = "mesh";

/// Resource with bundles to spawn
#[derive(Resource, Default)]
pub struct EditorUiReg {
    pub bundles : HashMap<String, HashMap<String, EditorBundleUntyped>>
}

impl EditorUiReg {
    pub fn add_bundle<T : Bundle + Clone>(&mut self, bundle : EditorBundle<T>) {
        
        let dyn_bundle = EditorBundleUntyped::new(bundle.data.clone(), bundle.name.clone());

        self.bundles.entry(bundle.category).or_default().insert(bundle.name.clone(), dyn_bundle);
    }
}

/// Contains all info to display and spawn editor bundle
pub struct EditorBundle<T : Bundle + Clone> {
    pub data : T,
    pub category : String,
    pub name : String
}

/// Untyped editor bundle
pub struct EditorBundleUntyped {
    pub data : Box<dyn Fn(&mut EntityCommands) + Send + Sync>,
    pub name : String
}

impl EditorBundleUntyped {
    /// Create new untyped editor bundle
    pub fn new<T : Bundle + Clone>(data : T, name : String) -> Self {
        Self {
            data : Box::new(move |cmds| {
                cmds.insert(data.clone());
            }),
            name 
        }
    }

    /// Spawn in world untyped editor bundle and mark entity as part of prefab
    pub fn spawn(&self, commands : &mut Commands) -> Entity {
        let mut cmds = commands.spawn_empty();
        (self.data)(&mut cmds);
        cmds.insert(PrefabMarker);
        cmds.id()
    }
}

/// Train to add editor_bundle(..) to App 
pub trait EditorUiExt {
    /// Register new bundle in editor ui
    fn editor_bundle<T : Bundle + Clone>(&mut self, category : &str, name : &str, bundle : T);
}

impl EditorUiExt for App {
    fn editor_bundle<T : Bundle + Clone>(&mut self, category : &str, name : &str, bundle : T) {
        let mut reg = if let Some(reg) = self.world.get_resource_mut::<EditorUiReg>() {
            reg
        } else {
            self.init_resource::<EditorUiReg>();
            self.world.get_resource_mut::<EditorUiReg>().unwrap()
        };

        reg.add_bundle(EditorBundle { data : bundle, category : category.to_string(), name : name.to_string() });
 
    }
}

/// Register all default editor bundles
pub fn register_default_editor_bundles(app : &mut App) {
    app.editor_bundle("Mesh", "Cube", (
        MeshPrimitivePrefab::Cube(1.0),
        Name::new("Cube".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Box", (
        MeshPrimitivePrefab::Box(BoxPrefab::default()),
        Name::new("Box".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Sphere", (
        MeshPrimitivePrefab::Sphere(SpherePrefab::default()),
        Name::new("UVSphere".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Quad", (
        MeshPrimitivePrefab::Quad(QuadPrefab::default()),
        Name::new("Quad".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Capsule", (
        MeshPrimitivePrefab::Capsule(CapsulePrefab::default()),
        Name::new("Capsule"),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Circle", (
        MeshPrimitivePrefab::Circle(CirclePrefab::default()),
        Name::new("Circle".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Cylinder", (
        MeshPrimitivePrefab::Cylinder(CylinderPrefab::default()),
        Name::new("Cylinder".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Icosphere", (
        MeshPrimitivePrefab::Icosphere(IcospherePrefab::default()),
        Name::new("Icosphere".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Plane", (
        MeshPrimitivePrefab::Plane(PlanePrefab::default()),
        Name::new("Plane".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "RegularPoligon", (
        MeshPrimitivePrefab::RegularPoligon(RegularPoligonPrefab::default()),
        Name::new("RegularPoligon".to_string()),
        Transform::default(),
        Visibility::default()
    ));
    app.editor_bundle("Mesh", "Torus", (
        MeshPrimitivePrefab::Torus(TorusPrefab::default()),
        Name::new("Torus".to_string()),
        Transform::default(),
        Visibility::default()
    ));

    app.editor_bundle("Camera", "CameraPlay", (
        Camera3d::default(),
        Name::new("Camera3d".to_string()),
        Transform::default(),
        Visibility::default(),
        CameraPlay::default()
    ));
}