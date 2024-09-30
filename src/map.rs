use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;



pub struct MapPlugin;

impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(LdtkWorldBundle{
        ldtk_handle: asset_server.load("map/roguelike_1.ldtk"),
        ..Default::default()
    });
}