use bevy::prelude::*;
use bevy_mod_picking::backends::raycast::RaycastPickable;
use bevy_mod_picking::prelude::*;

use crate::gui::panels::ui_selected_body::describe_body;
use crate::gui::tools::follow_body::click_body;
use crate::sol::celestial_body::CelestialBody;

// TODO:
// READ THIS
// https://bevy-cheatbook.github.io/programming/bundle.html#loose-components-as-bundles

// Bundles vs tuples

#[derive(Component)]
pub struct Simulated;

#[derive(Bundle)]
struct CelestialBodyBundle {
    body: CelestialBody,
    clicked: PickableBundle,
    raycast: RaycastPickable,
    click: On<Pointer<Click>>,
    hover: On<Pointer<Over>>,
    simulated: Simulated,
}

impl CelestialBodyBundle {
    fn new(
        body: CelestialBody,
    ) -> Self {
        Self {
            body,
            clicked: PickableBundle::default(),
            raycast: RaycastPickable,
            simulated: Simulated,
            click: On::<Pointer<Click>>::run(click_body),
            hover: On::<Pointer<Over>>::run(describe_body),
        }
    }
}

pub fn spawn_body(
    body: CelestialBody,
    commands: &mut Commands,
) {
    info!("Spawning {:?}", &body);
    commands.spawn(CelestialBodyBundle::new(body));
}
