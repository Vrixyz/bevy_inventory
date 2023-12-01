use crate::simple_mouse::MouseWorldPosition;

use super::*;
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                clear_build_requests,
                click_get_out,
                apply_deferred,
                (
                    verify_empty_space,
                    // TODO: add more checks
                    apply_deferred,
                    react_to_build,
                )
                    .run_if(component_exist::<BuildRequest>)
                    .chain(),
            )
                .chain(),
        );
    }
}

#[derive(Component)]
struct BuildRequest {
    pub inventory: Entity,
    pub item: Entity,
    pub position: Vec2,
}

#[derive(Component)]
enum RefusedBuild {
    NotEnoughPlace,
}

fn component_exist<T: Component>(q: Query<Entity, With<T>>) -> bool {
    q.iter().next().is_some()
}

fn click_get_out(
    mut commands: Commands,
    mut q_inventory: Query<(
        Entity,
        &mut inventory_generic::Inventory<crate::enemies::ItemType>,
    )>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_position_world: Res<MouseWorldPosition>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        for mut i in q_inventory.iter_mut() {
            let first = i.1.items.front().unwrap();

            commands.spawn(BuildRequest {
                inventory: i.0,
                item: *first,
                position: mouse_position_world.0,
            });
        }
    }
}

fn verify_empty_space(mut commands: Commands, q_requests: Query<(Entity, &BuildRequest)>) {
    for br in q_requests.iter() {
        info!("build at: {}", &br.1.position.x);
        if (0f32..100f32).contains(&br.1.position.x) {
            info!("forbidden");
            commands.entity(br.0).insert(RefusedBuild::NotEnoughPlace);
        }
    }
}

// TODO use cmponents and check everything ok
fn react_to_build(
    mut commands: Commands,
    mut q_inventory: Query<&mut inventory_generic::Inventory<crate::enemies::ItemType>>,
    build_events: Query<&BuildRequest, Without<RefusedBuild>>,
    mut q_transform: Query<&mut Transform>,
    mut rng: ResMut<RandomDeterministic>,
) {
    for event in build_events.iter() {
        let mut inventory = q_inventory.get_mut(event.inventory).unwrap();
        let item_index = inventory
            .items
            .iter()
            .position(|i| *i == event.item)
            .unwrap();
        inventory.items.remove(item_index);
        q_transform.get_mut(event.item).unwrap().translation = event.position.extend(0f32);
        let choices = [
            (enemies::ItemType::Gun, 2),
            (enemies::ItemType::Rifle, 1),
            (enemies::ItemType::Aura, 1),
        ];
        inventory.items.push_back(
            commands
                .spawn(choices.choose_weighted(&mut rng.random, |i| i.1).unwrap().0)
                .id(),
        );
    }
}
// TODO use cmponents and check everything ok
fn clear_build_requests(mut commands: Commands, build_events: Query<Entity, With<BuildRequest>>) {
    for e in build_events.iter() {
        commands.entity(e).despawn();
    }
}
