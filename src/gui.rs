use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2, Vec2}, EguiContexts, EguiPlugin};
use bevy_mod_picking::{prelude::{Click, Listener, Pointer, Over}, DefaultPickingPlugins};

use crate::sol::celestial_body::CelestialBody;

pub struct SolGuiPlugin;

impl Plugin for SolGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
        .insert_resource(UISSelectedBody::default())
            .add_plugins(DefaultPickingPlugins)
            .add_systems(Startup, setup_gui)
            .add_systems(Update, render_active_body_gui);
    }
}

fn setup_gui(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

#[derive(Resource)]
pub struct UISSelectedBody {
    pub selected: Option<Entity>,
}

impl Default for UISSelectedBody {
    fn default() -> Self {
        Self { selected: None }
    }
}

pub fn click_body() {
    // TODO: Left click follow
    // Right click delete??
}

pub fn describe_body(
    mut contexts: EguiContexts,
    mut active_body: ResMut<UISSelectedBody>,
    query: Query<&CelestialBody>,
    event: Listener<Pointer<Over>>,
    mut commands: Commands,
) {
    if let Ok(body) = query.get(event.target) {
        active_body.selected = Some(event.target.clone());
    }
}

fn render_active_body_gui(
    active_body: Res<UISSelectedBody>,
    query: Query<&CelestialBody>,
    mut egui_contexts: EguiContexts,
) {
    match active_body.selected {
        Some(selected) => {
            if let Ok(body) = query.get(selected) {
                egui::Window::new(format!("Info: {:?}", body.name))
                // .scroll2([false, true])
                // .vscroll(true)
                .fixed_size(Vec2 { x: 100.0, y: 100.0 })
                // .default_width(400.0)
                .resizable(false)
                .anchor(Align2::RIGHT_TOP, Vec2 { x: -10.0, y: 50.0 })
                .show(egui_contexts.ctx_mut(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Type: {:?}", body.body_type));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Name: {:?}", body.name));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Mass: {:?}", body.mass));
                    });
                    ui.horizontal(|ui| {
                        // TODO: What is this unit?
                        ui.label(format!("Speed: {:.4} mps", body.speed()));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Pos: x:{:.4}, y:{:.4}", body.pos.x, body.pos.y));
                    });
                });
            }
        }
        None => (),
    };
}