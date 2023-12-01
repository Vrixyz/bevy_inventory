use super::HIGHLIGHT_TINT;
use super::ITEM_VISUAL_SIZE;
use crate::inventory_generic::*;
use bevy::ecs::system::EntityCommand;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::HashMap;
use bevy_mod_picking::PickableBundle;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::inventory_generic::InventoryPlugin::<ItemType>::default());
        app.add_systems(Startup, (create_assets, spawn_layout).chain());
    }
}

#[derive(Resource)]
pub struct ItemDef {
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
}

impl ItemDef {
    pub(crate) fn create_item_visual(
        &self,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        bevy_mod_picking::prelude::Highlight<ColorMaterial>,
        PickableBundle,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: self.mesh.clone(),
                transform: Transform::default().with_scale(Vec3::splat(ITEM_VISUAL_SIZE)),
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

pub(crate) fn create_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let quad: Mesh2dHandle = meshes.add(Mesh::from(shape::Quad::default())).into();
    commands.insert_resource(VisualAssets {
        item_def: [
            (
                ItemType::Gun,
                ItemDef {
                    mesh: quad.clone(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                },
            ),
            (
                ItemType::Rifle,
                ItemDef {
                    mesh: quad.clone(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW)),
                },
            ),
            (
                ItemType::Aura,
                ItemDef {
                    mesh: quad.clone(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                },
            ),
        ]
        .into(),
    });
}

pub(crate) fn spawn_layout(mut commands: Commands) {
    let inventory = vec![
        commands.spawn(ItemType::Gun).id(),
        commands.spawn(ItemType::Rifle).id(),
        commands.spawn(ItemType::Aura).id(),
    ]
    .into();
    commands.spawn((
        Inventory::<ItemType> {
            items: inventory,
            ..default()
        },
        InventoryVisualDef {
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

impl CommandVisualBuilder for ItemType {
    type C = CreateItemDefVisual;
    fn command_to_create_visual(&self) -> Self::C {
        CreateItemDefVisual { item_type: *self }
    }
}

pub struct CreateItemDefVisual {
    pub item_type: ItemType,
}

impl EntityCommand for CreateItemDefVisual {
    fn apply(self, id: Entity, world: &mut World) {
        let assets = world.get_resource::<VisualAssets>().unwrap();
        let def = &assets.item_def[&self.item_type];
        let visual = def.create_item_visual();
        world.entity_mut(id).insert(visual);
    }
}
