use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_level);
    }
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MapBundle {
        map: asset_server.load("levels/prototype.map"),
        ..default()
    });
}
