use bevy::prelude::*;
use std::collections::VecDeque;

use super::ItemType;
use super::VisualAssets;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, item_create_visual);
    }
}

pub trait ItemVisualSource<T>
where
    T: Bundle,
{
    // When impl trait in traits, T impl Bundle and get itemDef out of inventory.
    fn create_item_visual(&self, position: Vec3) -> T;
}

#[derive(Component)]
pub struct MarkerItemVisual;

#[derive(Component)]
pub struct Inventory {
    /// entities contained here have a MarkerItem component, it handles logic
    /// their rendering is created via item_create_visual
    pub items: VecDeque<Entity>,
}

#[derive(Component)]
pub struct InventoryVisualDef {
    pub positions: Vec<Vec3>,
}

fn item_create_visual(
    mut commands: Commands,
    assets: Res<VisualAssets>,
    inventory: Query<(&Inventory, &InventoryVisualDef), Changed<Inventory>>,
    items_without_visual: Query<(Entity, &ItemType, Has<MarkerItemVisual>)>,
    mut q_transform: Query<&mut Transform>,
) {
    for (inventory, visual_def) in inventory.iter() {
        for (i, item) in inventory
            .items
            .iter()
            .take(visual_def.positions.len())
            .enumerate()
        {
            let Ok(item) = items_without_visual.get(*item) else {
                continue;
            };
            if item.2 {
                q_transform.get_mut(item.0).unwrap().translation = visual_def.positions[i];
                continue;
            }
            let assets = &assets.item_def[item.1];
            let position = visual_def.positions[i];
            let mut c = commands.entity(item.0);
            c.insert((assets.create_item_visual(position), MarkerItemVisual));
        }
    }
}
