mod boundry;
mod gui;
mod sol;

use crate::boundry::spawn_body;
use crate::gui::panels::ui_time::GameTimePlugin;
use crate::sol::celestial_body::CelestialBody;
use crate::sol::reality_calulator::{calculate_new_positions, default_system, one_planet_system};

use bevy::prelude::*;
use boundry::Simulated;
// use bevy_mod_picking::prelude::*;
use gui::SolGuiPlugin;
use gui::panels::ui_time::TimeState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        // Time
        .add_plugins(GameTimePlugin)
        // GUI, Controls, Camera
        .add_plugins(SolGuiPlugin)
        // Background
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 255.0,
        }))
        // Insert universe
        .add_systems(Startup, big_bang)
        // Systems to run every frame
        .add_systems(Update, update_positions)
        .run();
}

fn big_bang(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("BANG");
    for body in one_planet_system().clone() {
        spawn_body(body, &mut commands, &mut meshes, &mut materials);
    }
}

fn update_positions(
    time: Res<Time>,
    time_state: ResMut<TimeState>,
    mut query: Query<(&mut CelestialBody, &mut Transform), With<Simulated>>,
) {
    let bodies: Vec<CelestialBody> = query.iter().map(|(body, _)| body.clone()).collect();
    let cloned = bodies.clone();
    let new_positions = calculate_new_positions(&bodies, cloned);

    // println!("Simulated Bodies: {:?}", bodies.len());

    for (mut body, mut transform) in query.iter_mut() {
        let new_body = new_positions
            .iter()
            .find(|x| x.name == body.name)
            .unwrap()
            .clone();

        body.pos = new_body.pos;
        body.momentum = new_body.momentum;
        transform.translation = Vec3::new(body.pos.x, body.pos.y, 0.);
    }
}
