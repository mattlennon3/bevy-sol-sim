use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_picking::prelude::{Click, Listener, Pointer};

use crate::sol::celestial_body::CelestialBody;

pub struct SolGuiPlugin;

impl Plugin for SolGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(UISSelectedBody::default())
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

pub fn click_body(
    mut contexts: EguiContexts,
    mut local: ResMut<UISSelectedBody>,
    // mut query: Query<(Entity, &Interaction)>,
    query: Query<&CelestialBody>,
    event: Listener<Pointer<Click>>,
    mut commands: Commands,
) {
    if let Ok(body) = query.get(event.target) {
        info!("Clicked Body: {:?}", body);
        local.selected = Some(event.target.clone());
    }
}

fn render_active_body_gui(
    local: Res<UISSelectedBody>,
    query: Query<&CelestialBody>,
    mut egui_contexts: EguiContexts,
) {
    match local.selected {
        Some(selected) => {
            if let Ok(body) = query.get(selected) {
                
            }
        }
        None => (),
    };
}