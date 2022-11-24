use bevy::prelude::{ResMut,Transform};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
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
    .add_plugin(camera::CustomCameraPlugin)
    .add_plugin(scene::CustomScene)
    .add_plugin(player::CustomPlayer)
    .run();
}



