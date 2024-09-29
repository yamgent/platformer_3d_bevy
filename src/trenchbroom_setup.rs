use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub struct TrenchBroomSetupPlugin;

impl Plugin for TrenchBroomSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TrenchBroomPlugin::new(trenchbroom_config()));

        if cfg!(windows) && cfg!(debug_assertions) {
            app.add_systems(Startup, write_trenchbroom_config);
        }
    }
}

fn trenchbroom_config() -> TrenchBroomConfig {
    TrenchBroomConfig::new("platformer_3d_demo_bevy")
        .entity_scale_expression("scale")
        .entity_definitions(entity_definitions!(
            Solid worldspawn {} |world, entity, view| {
                view.spawn_brushes(world, entity, BrushSpawnSettings::new().smooth_by_default_angle().pbr_mesh());
            }
        ))
}

fn write_trenchbroom_config(config: Res<TrenchBroomConfig>) {
    let folder_path = "trenchbroom/config/games/platformer_3d_demo_bevy/";
    if let Err(err) = config.write_folder(folder_path) {
        error!("Could not write TrenchBroom config: {err}");
    }
    info!("Finish writing trenchbroom config to {folder_path}");
}
