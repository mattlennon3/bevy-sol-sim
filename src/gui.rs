pub mod ui_camera;
pub mod ui_follow_body;
pub mod ui_selected_body;

use bevy::prelude::*;
// use bevy_mod_picking::prelude::*;

use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_egui::egui::{self};
use bevy_fly_camera::FlyCameraPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

use self::ui_selected_body::{render_active_body_gui, UISelectedBody};
use self::ui_follow_body::{follow_body, UIFollowBody};
use self::ui_camera::{setup_camera, zoom_2d};

pub struct SolGuiPlugin;

impl Plugin for SolGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_plugins(FlyCameraPlugin)
            .add_plugins(DefaultPickingPlugins)
            .insert_resource(UISelectedBody::default())
            .insert_resource(UIFollowBody::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Startup, setup_gui)
            .add_systems(Update, render_active_body_gui)
            .add_systems(Update, zoom_2d)
            .add_systems(Update, follow_body);
    }
}

fn setup_gui(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}
