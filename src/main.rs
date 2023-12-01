mod inventory_buildings_interaction;
mod inventory_generic;
mod simple_mouse;
pub mod buildings;
pub mod enemies;
mod inventory_enemies_interaction;


use bevy::{
    core_pipeline::bloom::BloomSettings,
    ecs::{
        schedule::{LogLevel, ScheduleBuildSettings},
        system::EntityCommand,
    },
    math::{vec3, vec4},
    prelude::*,
    sprite::{},
    utils::HashMap,
};
use bevy_mod_picking::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
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
        app.add_plugins(inventory_buildings_interaction::DebugPlugin);
        app.add_plugins(inventory_enemies_interaction::DebugPlugin);
        app.add_plugins(simple_mouse::MousePlugin);
        app.add_plugins(buildings::Plugin);
        app.add_plugins(enemies::Plugin);
        app.add_systems(Startup, spawn_camera);
        app.init_resource::<RandomDeterministic>();
    }
}

fn spawn_camera(mut commands: Commands) {
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
}

