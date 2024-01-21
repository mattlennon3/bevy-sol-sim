use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::backends::raycast::RaycastPickable;

use crate::gui::panels::ui_selected_body::describe_body;
use crate::gui::tools::follow_body::click_body;

// pub struct CelestialBodyGuiPlugin;

// impl Plugin for CelestialBodyGuiPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, )
//         app.add_system_set(
//             SystemSet::on_enter(crate::AppState::Running)
//                 .with_system(setup_gui.system()),
//         );
//     }
// }

#[derive(Bundle)]
pub struct CelestialBodyGuiBundle {
    pub raycast: RaycastPickable,
    pub clickable: PickableBundle,
    pub click: On<Pointer<Click>>,
    pub hover: On<Pointer<Over>>,
}

impl CelestialBodyGuiBundle {
    pub fn new() -> Self {
        Self {
            clickable: PickableBundle::default(),
            raycast: RaycastPickable,
            click: On::<Pointer<Click>>::run(click_body),
            hover: On::<Pointer<Over>>::run(describe_body),
        }
    }
}
