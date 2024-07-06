use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub star: Handle<Scene>,
    pub planet: Handle<Scene>,
    pub asteroid: Handle<Scene>,
}

//TODO: See what pub(crate does)
pub(crate) struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    let solar_system: Handle<Scene> = asset_server.load("downloads/solar_system/SolarSystem.glb#Scene0");
    
    *scene_assets = SceneAssets {
        star: solar_system,
        planet: asset_server.load("Planet_3.fbx"),
        asteroid: asset_server.load(""),
    }
}
