mod inventory;
mod inventory_debug;
mod simple_mouse;

use inventory::{ItemVisualSource, MarkerItemVisual};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::collections::VecDeque;

use bevy::{
    core_pipeline::bloom::BloomSettings,
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    math::{vec3, vec4},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::HashMap,
};
use bevy_mod_picking::prelude::*;
use simple_mouse::MainCamera;

const ITEM_VISUAL_SIZE: f32 = 64f32;
const HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED: Color = Color::rgb(0.35, 0.75, 0.35);

// We can use a dynamic highlight that builds a material based on the entity's base material. This
// allows us to "tint" a material by leaving all other properties - like the texture - unchanged,
// and only modifying the base color. The highlighting plugin handles all the work of caching and
// updating these materials when the base material changes, and swapping it out during pointer
// events.
//
// Note that this works for *any* type of asset, not just bevy's built in materials.
const HIGHLIGHT_TINT: Highlight<ColorMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| ColorMaterial {
        color: HOVERED,
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| ColorMaterial {
        color: PRESSED,
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| ColorMaterial {
        color: matl.color * vec4(5.2, 5.2, 5.2, 1.0),
        ..matl.to_owned()
    })),
};
fn main() {
    App::new()
        .edit_schedule(Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [800., 600.].into(),
                title: "Bevy CSS Grid Layout Example".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(InventoryPlugin)
        .run();
}

#[derive(Resource)]
pub struct RandomDeterministic {
    pub random: ChaCha20Rng,
    pub seed: u64,
}
impl Default for RandomDeterministic {
    fn default() -> Self {
        let seed = 0; //thread_rng().gen::<u64>();
        Self {
            random: ChaCha20Rng::seed_from_u64(seed),
            seed,
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(inventory_debug::DebugPlugin);
        app.add_plugins(simple_mouse::MousePlugin);
        app.add_plugins(inventory::InventoryPlugin);
        app.add_systems(Startup, (create_assets, spawn_layout).chain());
        app.init_resource::<RandomDeterministic>();
    }
}

#[derive(Resource)]
pub struct ItemDef {
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
}

impl
    ItemVisualSource<(
        MaterialMesh2dBundle<ColorMaterial>,
        bevy_mod_picking::prelude::Highlight<ColorMaterial>,
        PickableBundle,
    )> for ItemDef
{
    fn create_item_visual(
        &self,
        position: Vec3,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        bevy_mod_picking::prelude::Highlight<ColorMaterial>,
        PickableBundle,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: self.mesh.clone(),
                transform: Transform::default()
                    .with_translation(position)
                    .with_scale(Vec3::splat(ITEM_VISUAL_SIZE)),
                material: self.material.clone(),
                ..default()
            },
            HIGHLIGHT_TINT,
            PickableBundle::default(), // <- Makes the mesh pickable.
        )
    }
}

#[derive(Resource)]
pub struct VisualAssets {
    pub item_def: HashMap<ItemType, ItemDef>,
}

fn create_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(VisualAssets {
        item_def: [
            (
                ItemType::Gun,
                ItemDef {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                },
            ),
            (
                ItemType::Rifle,
                ItemDef {
                    mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW)),
                },
            ),
            (
                ItemType::Aura,
                ItemDef {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                },
            ),
        ]
        .into(),
    });
}

fn spawn_layout(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::default(),
        MainCamera,
    ));

    let inventory = vec![
        commands.spawn(ItemType::Gun).id(),
        commands.spawn(ItemType::Rifle).id(),
        commands.spawn(ItemType::Aura).id(),
    ]
    .into();
    commands.spawn((
        inventory::Inventory { items: inventory },
        inventory::InventoryVisualDef {
            positions: vec![
                vec3(0f32, 0f32, 0f32),
                vec3(0f32, ITEM_VISUAL_SIZE + 10f32, 0f32),
                vec3(0f32, (ITEM_VISUAL_SIZE + 10f32) * 2f32, 0f32),
            ],
        },
    ));
}

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ItemType {
    Gun,
    Rifle,
    Aura,
}
