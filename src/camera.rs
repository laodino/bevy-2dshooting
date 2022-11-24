use bevy::prelude::*;
pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin{
    fn build(&self,app:&mut App){
          app.add_startup_system(setcamera);
    }
}
 
fn setcamera(mut commands:Commands){
    commands.spawn(Camera2dBundle{..default()});
}