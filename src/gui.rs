use bevy::{prelude::*, ecs::system::EntityCommands, transform::commands};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};
use bevy_mod_picking::prelude::{ListenerInput, Pointer, Click, Listener};

use crate::sol::celestial_body::CelestialBody;

pub struct SolGuiPlugin;

impl Plugin for SolGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, setup_gui);
            // .add_systems(Update, gui_update);
            // .add_systems(Update, selected_planet);
    }
}

fn setup_gui(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn click_body(
    mut contexts: EguiContexts,
    mut local: Local<UISSelectedBody>,
    // mut query: Query<(Entity, &Interaction)>,
    query: Query<&CelestialBody>,
    event: Listener<Pointer<Click>>,
    mut commands: Commands
    ) {
        if let Ok(body) = query.get(event.target) {
            info!("click_body {:?}", body);
        }
        // info!("entty {:?}", entity)
        // for (entity, interaction) in query.iter_mut() {
        // }
}


// pub struct UISSelectedBody<'a>(&'a CelestialBody);
pub struct UISSelectedBody {
    pub selected: Option<Entity>
}

impl Default for UISSelectedBody {
    fn default() -> Self {
        Self {
            selected: None
        }
    }
}

// pub fn selected_planet<T>(
//     listener: Res<ListenerInput<T>>, entity: &mut EntityCommands
//     // mut local: Local<UISSelectedBody>,
// ) {
//     // if(local.selected.is_none()) {
//     //     return;
//     // }
//     // info!("SELECTED PLANET {:?}", local.selected);
// }

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
