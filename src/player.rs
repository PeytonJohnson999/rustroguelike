use benimator::FrameRate;
use bevy::{core_pipeline::experimental::taa, math::uvec2, prelude::*, render::texture};
use crate::components::{self, *};
use crate::colliders::ColliderBundle;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::ground_detection::*;

const MAP_FLOOR: f32 = 0.0;
const BASE_PLAYER_SPEED: f32 = 120.0;

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Component, Default, LdtkEntity)]
pub struct PlayerMarker{
    l : i32,
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle{
    // Might have to change playerbuilder to customize the spritesheet, worldly, and entityinstance
    pub sprite_bundle : LdtkSpriteSheetBundle,
    pub player : Player,
    pub anim_config : AnimationConfig,
    pub animation : Animation,
    pub animation_state: AnimationState,
    pub worldly: Worldly,
    pub collider_bundler: ColliderBundle,
    pub ground_detection: GroundDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

impl Default for PlayerBundle{
    fn default() -> Self {
        Self { 
            sprite_bundle: LdtkSpriteSheetBundle::default(), 
            player: Player, 
            anim_config: AnimationConfig::new(0, 3, 12), 
            animation: Animation::default(),
            animation_state: AnimationState::default(),
            worldly: Worldly::default(),
            collider_bundler: ColliderBundle::default(),
            ground_detection: GroundDetection::default(),
            entity_instance: EntityInstance::default(),
        }
    }
}

impl std::fmt::Debug for PlayerBundle{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerBundle")
            .field("SpriteBundle", dbg!(&self.sprite_bundle))
            .finish()
            
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup);
        
        app
            .register_ldtk_entity::<PlayerMarker>("Player")
            .add_systems(Update, player_builder)
            .add_systems(FixedUpdate, (move_player/*, apply_velocity, apply_gravity*/))
            
            ;
            
    }
}

// pub fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
// ){
//     // unimplemented!();
//     // //Textures
//     // let layout = TextureAtlasLayout::from_grid(uvec2(16, 23), 1, 4, None, None);
//     // let texture_atlas_layout = texture_atlas_layout.add(layout);

//     // //Animation config
//     // let anim_config = AnimationConfig::new(0, 3, 24);

//     //Benimator animation
//     // let animation = Animation(benimator::Animation::from_indices(0..=3, FrameRate::from_fps(24.0)));

// }


//Controls
pub fn move_player(
    mut query : Query<(
                    &mut Velocity, // From ColliderBody
                    &mut TextureAtlas, // From LdtkSpriteSheetBundle
                    &mut AnimationState,
                    &mut Sprite, // From LdtkSpriteSheetBundle::SpriteBundle
                    &GroundDetection,
                    &Animation), 
                    With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
){
    
    let (mut vel, mut textures, 
        mut anim_state, mut sprite, ground_detection, animation) = query.single_mut();
    
    let right = if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight){1.} else {0.};
    let left = if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {1.} else{0.};

    vel.linvel.x = (right - left) * 200.;

    if keyboard.pressed(KeyCode::Space) && (ground_detection.on_ground){
        vel.linvel.y = 500.;
    }

    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight){
        if sprite.flip_x {
            sprite.flip_x = false;
        }
        anim_state.update(animation, timer.delta());
        textures.index = anim_state.frame_index();
        vel.linvel.x = (right - left) * 200.;
    }
    if (keyboard.just_released(KeyCode::KeyD) || keyboard.just_released(KeyCode::ArrowRight))
         && vel.linvel.x >= 0.{
        anim_state.update(animation, timer.delta());
        textures.index = 0;
        vel.linvel.x = 0.;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft){
        if !sprite.flip_x {
            sprite.flip_x = true;
        }
        anim_state.update(animation, timer.delta());
        textures.index = anim_state.frame_index();
        vel.linvel.x = (right - left) * 200.;
    }
    if (keyboard.just_released(KeyCode::KeyA) || keyboard.just_released(KeyCode::ArrowLeft))
         && vel.linvel.x <= 0.{
        anim_state.update(animation, timer.delta());
        textures.index = 0;
        vel.linvel.x = (right - left) * 0.;
    }
}


fn player_builder(
    mut commands: Commands,
    mut query: Query<Entity, With<PlayerMarker>>,
){
    println!("Player builder called");
    for (entity) in query.iter_mut(){
        // println!("Player built!");
        // commands.entity(entity).with_children(|builder|{
        //     builder
        //         .spawn_empty()
        //         .insert(PlayerBundle{
        //             anim_config: AnimationConfig::new(0, 3, 12),
        //             ..Default::default()
        //         })
        //         ;
        // // });
        println!("Entity {}", entity);
    }
}


