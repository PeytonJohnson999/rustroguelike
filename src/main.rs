#![allow(warnings)]
use std::default;

use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::utils::dbg;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;

#[derive(Clone, Copy, Eq, PartialEq, Default, States, Debug, Hash)]
enum GameState {
    MainMenu,
    Menu,
    #[default]
    LoadingScreen,
    Game,
}

mod player;
mod components;
mod gui;
mod map;
mod colliders;
mod ground_detection;

fn main(){
    App::new().add_plugins((
        DefaultPlugins.set(AssetPlugin{
            watch_for_changes_override : Some(true),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest()),
        Wireframe2dPlugin,
        LdtkPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        map::MapPlugin,
        player::PlayerPlugin,
    ))
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()})
        .insert_state(GameState::LoadingScreen)
        .add_systems(Startup, (setup, gui::setup).chain())
        .add_systems(Update, (toggle_wireframe))
        .run();
    
}


const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
){
    if let Ok(mut window) = window_query.get_single_mut(){
        window.title = "Roguelike".to_string();
    }
    commands.spawn(Camera2dBundle::default());

    
}


fn setup_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let shapes = [
        Mesh2dHandle(meshes.add(Circle{radius: 50.0})),
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];

    let num_shapes = shapes.len();

    for ( i, shape) in shapes.into_iter().enumerate(){
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
        commands.spawn(MaterialMesh2dBundle{
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                50.0,
                0.0,
            ),
            ..default()
        });
    }

    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
            .with_style(Style{
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );

    

}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
){
    if keyboard.just_pressed(KeyCode::Space){
        wireframe_config.global = !wireframe_config.global;
    }
}






