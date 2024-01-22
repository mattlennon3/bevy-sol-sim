mod gui;
mod sol;

use crate::gui::panels::ui_time::GameTimePlugin;
use crate::gui::sol_gui::celestial_body_gui::CelestialBodyGuiBundle;
use crate::sol::reality_calculator::Simulated;
use crate::sol::systems::{one_planet_system, sol_system, twin_planet_system};

use bevy::prelude::*;
use gui::SolGuiPlugin;
use sol::reality_calculator::RealityCalculatorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        // GUI, Controls, Camera
        .add_plugins(SolGuiPlugin)
        // Time
        .add_plugins(GameTimePlugin)
        // Universe
        .add_plugins(RealityCalculatorPlugin)
        // Background
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 255.0,
        }))
        // Insert universe
        .add_systems(Startup, big_bang)
        .run();
}

// Move to systems.rs?
fn big_bang(mut commands: Commands) {
    let system = twin_planet_system();
    info!("BANG");
    for (body_bundle, momentum) in system.clone() {
        // info!("Spawning {:?}", &body_bundle);
        commands
            .spawn(body_bundle)
            .insert(momentum)
            .insert((CelestialBodyGuiBundle::new(), Simulated));
    }
    info!("Simulated Bodies: {:?}", system.len());
}
