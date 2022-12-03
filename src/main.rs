use bevy::prelude::{ResMut,Transform};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
use bevy_inspector_egui::WorldInspectorPlugin;
pub mod camera;
pub mod player;
pub mod scene; 
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "BlueWizardAdventure!".to_string(),
            width: 1366.,
            height: 768.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }))
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(camera::CustomCameraPlugin)
    .add_plugin(scene::CustomScenePlugin)
    .add_plugin(player::CustomPlayerPlugin)
    .run();
}



