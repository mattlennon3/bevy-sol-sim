use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::sol::celestial_body::CelestialBody;

pub struct SolGuiPlugin;

impl Plugin for SolGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            // .add_systems(Startup, setup_gui)
            .add_systems(Update, gui_update);
    }
}

fn setup_gui(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

// TODO: DOES NOT WORK

fn gui_update(
    mut commands: Commands,       
    mut selected_query: Query<Entity>,
    mut interaction_query: Query<(&Interaction, Entity)>,
) {
    for (interaction, entity) in &interaction_query {
        info!("GUI UPDATE");
        match interaction {
            Interaction::Pressed => {
                // Handle click event
                info!("GUI Pressed");

                // commands.entity(entity).despawn();
            }

            Interaction::Hovered => {
                info!("GUI Hovered");
            }
            _ => {
                info!("GUI NONE");
            }
        }
    }
}
