use bevy::prelude::ResMut;
use bevy::prelude::*;
use std::path::Path;

#[derive(Resource,Default)]
struct PlayerAssets {
    idlehandles: Vec<HandleUntyped>,
    walkhandles: Vec<HandleUntyped>,
    jumphandles: Vec<HandleUntyped>,
}

pub struct CustomPlayer;

impl Plugin for CustomPlayer {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_assets_loader)
            .add_startup_system(setup_player);
    }
}

fn player_assets_loader(mut commands: Commands, server: Res<AssetServer>) {
    //use
    if let Ok(handles) = server.load_folder("BlueWizard Animations\\BlueWizard\\2BlueWizardIdle") {
        commands.insert_resource(PlayerAssets(handles));
        print!("load success");
    } else {
        print!("load failed");
    };
}

fn setup_player(mut commands: Commands, server: Res<AssetServer>) {
    let bwizard_handle = server.get_handle(
        "BlueWizard Animations\\BlueWizard\\2BlueWizardIdle\\Chara - BlueIdle00000.png",
    );

    commands.spawn(SpriteBundle {
        texture: bwizard_handle,
        transform: Transform::from_xyz(50., 50., 0.),
        ..default()
    });
}
