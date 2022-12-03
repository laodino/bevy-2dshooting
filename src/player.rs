use bevy::prelude::ResMut;
use bevy::{asset::LoadState, prelude::*};
use bevy_rapier2d::geometry::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

const PLAYERASSETSPATH:&str= "BlueWizard Animations\\player.png";
const SPRITESIZEWIDTH:f32 = 532.;
const SPRITESIZEHEIGHT:f32 = 552.;
const SPRITECOLS:usize =20;
const SPRITEROWS:usize =3;

#[derive(Resource, Default)]
struct PlayerAssets {
    playerassethandles: Handle<TextureAtlas>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}
enum PlayerState {
    IDLE,
    Walk,
    Jump,
}
#[derive(PartialEq, Clone, Copy)]
enum PlayerDirection {
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Jumper {
    linevelocity: f32,
    isjumping: bool,
}

#[derive(Component)]
struct Player {
    playerstate: PlayerState,
    direction: PlayerDirection,
}

pub struct CustomPlayerPlugin;

impl Plugin for CustomPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_state(AppState::Setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(player_assets_loader))
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_assets_load))
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(spawn_player))
            .add_system(animate_sprite)
            .add_system(player_jump)
            .add_system(jump_reset)
            .add_system(player_movement)
            ;
    }
}

fn player_assets_loader(mut commands: Commands,
     server: Res<AssetServer>,
     mut texture_atlases:ResMut<Assets<TextureAtlas>>,
    ) {
    //use
    let texture_handle = server.load(PLAYERASSETSPATH);
    let texture_atlas =TextureAtlas::from_grid(
        texture_handle,
         Vec2::new(SPRITESIZEWIDTH,SPRITESIZEHEIGHT), 
         SPRITECOLS, 
         SPRITEROWS, 
         None,
          None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(PlayerAssets {
        playerassethandles: texture_atlas_handle ,
    });
}

fn check_assets_load(
    mut state: ResMut<State<AppState>>,
    spritehandles: Res<PlayerAssets>,
    server: Res<AssetServer>,
) {
    //it seems this function do not work
    //if let LoadState::Loaded = server.get_load_state(&spritehandles.playerassethandles) {
        state.set(AppState::Finished).unwrap();
    //}
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Player
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle,player) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
               let  indexinfo:(usize,usize) = match player.playerstate {
                   PlayerState::IDLE=>(0,20),
                   PlayerState::Walk=>(19,20),
                   PlayerState::Jump=>(39,7),
               };


            //let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % indexinfo.1+indexinfo.0;
            
        }
    }
}


fn spawn_player(
    mut commands: Commands,
    spritehandles: Res<PlayerAssets>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: spritehandles.playerassethandles.clone(),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 1.,
                    },
                    scale: Vec3::splat(0.2),
                    //face to left
                   // rotation: Quat::from_rotation_y(PI),
                    ..default()
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED_Z)
        .insert(Collider::cuboid(10., 20.))
        .insert(GravityScale(1.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: Vec2 { x: 0., y: 0. },
            ..default()
        })
        .insert(Jumper {
            linevelocity: 200.0,
            isjumping: false,
        })
        .insert(Player {
            playerstate: PlayerState::IDLE,
            direction: PlayerDirection::Right,
        });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut playerquery: Query<(&mut Velocity, &mut Player,&mut TextureAtlasSprite), With<Player>>,
) {
    if let Some((mut velocity, mut player,mut textureatlasprite)) = playerquery.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::A) {
            player.direction = PlayerDirection::Left;
            player.playerstate = PlayerState::Walk;
            //flippy
            //  let old_rotation =  transform.rotation ;
           // transform.rotation = transform.rotation.inverse();
            //  transform.scale=Vec3::splat(0.6);
            textureatlasprite.flip_x=true;
            velocity.linvel = Vec2::new(-100., velocity.linvel.y).into();
        } else if keyboard_input.pressed(KeyCode::D) {
            player.direction = PlayerDirection::Right;
            player.playerstate = PlayerState::Walk;
            textureatlasprite.flip_x=false;
            velocity.linvel = Vec2::new(100., velocity.linvel.y).into();
        }else
        {
         //   player.playerstate = PlayerState::IDLE;
        }
        ;
    }
}

fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut jumperquery: Query<(&mut Jumper, &mut Velocity,&mut Player), With<Player>>,
) {
    if let Some((mut jumper, mut velocity,mut player)) = jumperquery.iter_mut().next() {
        if keyboard_input.any_pressed([KeyCode::Space, KeyCode::W]) && !jumper.isjumping {
            velocity.linvel = Vec2::new(velocity.linvel.x, jumper.linevelocity).into();
            jumper.isjumping = true;
            player.playerstate = PlayerState::Jump;
        }
    } else {
        print!("not found");
    }
}

fn jump_reset(
    mut colliderevent: EventReader<CollisionEvent>,
    mut jumper: Query<(Entity, &mut Jumper,&mut Player)>,
) {
    for contact_event in colliderevent.iter() {
        for (entity, mut jumper,mut player) in jumper.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    jumper.isjumping = false;
                   // player.playerstate = PlayerState::IDLE;
                }
            }
        }
    }
}
