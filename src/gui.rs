pub mod assets;
pub mod camera;
pub mod constants;
pub mod kb_mouse;
pub mod panels;
pub mod tools;

use bevy::prelude::*;
// use bevy_mod_picking::prelude::*;

use bevy::window::{WindowRef, WindowResolution};
use bevy_egui::egui::{self};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_mod_picking::prelude::EntityEvent;
use bevy_mod_picking::DefaultPickingPlugins;

use self::assets::mesher_plugin::{self, MesherPlugin};
use self::kb_mouse::mouse_states::UIMouseState;
use self::panels::ui_selected_body::{render_active_body_gui, UISelectedBody};
// use self::bottom_panel::{UIPickedBody};
use self::assets::asset_loader::AssetLoaderPlugin;
use self::camera::ui_camera::{setup_camera, zoom_2d};
use self::panels::ui_bottom_panel::{render_bottom_panel_gui, UIPickedBody};
use self::tools::follow_body::{follow_body, UIFollowBody};
use self::tools::spawning::SpawningPlugin;
use self::tools::trajectory::TrajectoryPlugin;

pub struct SolGuiPlugin;

impl Plugin for SolGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_window_size)
            // .add_plugins(EguiPlugin)
            .add_plugins(WorldInspectorPlugin::new()) // instead of egui
            .add_plugins(FlyCameraPlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(SpawningPlugin)
            .add_plugins(TrajectoryPlugin)
            .add_plugins(AssetLoaderPlugin)
            .add_plugins(MesherPlugin)
            .insert_resource(UISelectedBody::default())
            .insert_resource(UIFollowBody::default())
            .insert_resource(UIPickedBody::default())
            .insert_resource(UIMouseState::default())
            .add_systems(Startup, setup_camera)
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

fn setup_window_size(mut windows: Query<&mut Window>) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };
    // println!("Window height: {:?}", window.physical_height());
    // println!("Window width: {:?}", window.physical_width());
    // println!("Window height: {:?}", window.height());
    // println!("Window width: {:?}", window.width());
    window.position.set(IVec2::new(0, 0));
    // window.resolution = WindowResolution::new(
    //     window.resolution.physical_width() as f32,
    //     (window.resolution.physical_height() as f32) * 0.9,
    // );
    // window.position = WindowPosition::new(IVec2::new(0, 0));
    // println!("scale_factor: {:?}", window.scale_factor());
}
