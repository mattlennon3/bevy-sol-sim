pub mod kb_mouse;
pub mod camera;
pub mod assets;
pub mod panels;
pub mod constants;
pub mod tools;

use bevy::prelude::*;
// use bevy_mod_picking::prelude::*;

use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_egui::egui::{self};
use bevy_fly_camera::FlyCameraPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

use self::kb_mouse::mouse_states::UIMouseState;
use self::panels::ui_selected_body::{render_active_body_gui, UISelectedBody};
// use self::bottom_panel::{UIPickedBody};
use self::panels::ui_bottom_panel::{render_bottom_panel_gui, UIPickedBody};
use self::tools::follow_body::{follow_body, UIFollowBody};
use self::tools::spawning::SpawningPlugin;
use self::camera::ui_camera::{setup_camera, zoom_2d};
use self::assets::asset_loader::AssetLoaderPlugin;

pub struct SolGuiPlugin;

impl Plugin for SolGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_plugins(FlyCameraPlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(SpawningPlugin)
            .insert_resource(UISelectedBody::default())
            .insert_resource(UIFollowBody::default())
            .insert_resource(UIPickedBody::default())
            .insert_resource(UIMouseState::default())
            .add_systems(Startup, setup_camera)
            .add_plugins(AssetLoaderPlugin)
            .add_systems(Startup, setup_gui)
            .add_systems(Update, render_active_body_gui)
            .add_systems(Update, render_bottom_panel_gui)
            .add_systems(Update, zoom_2d)
            .add_systems(Update, follow_body);
    }
}

fn setup_gui(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 5.0.into(),
        ..Default::default()
    });
}
