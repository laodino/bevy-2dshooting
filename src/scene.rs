use bevy::prelude::*;
pub struct CustomScene;

impl Plugin for CustomScene{
    fn build(&self,app:&mut App){
        app
        .add_startup_system(load_scene_assets)
       ;
    }
}

fn load_scene_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Mossy Assets\\Mossy Tileset\\Mossy - FloatingPlatforms.png");
    let texture_atlas =
        TextureAtlas::from_grid(
            texture_handle,
             Vec2::new(2048.0, 2048.*240./1000.),
              1,
               1, 
               None,
                Some(Vec2::new(0.,2048.*760./1000.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                 translation: Vec3 { x: 0., y: -250., z: 0. },
                  scale:Vec3::splat(0.5),
                  ..default()
                 },
            ..default()
        },   
    ));
}