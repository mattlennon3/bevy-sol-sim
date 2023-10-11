mod gui;
mod sol;
mod boundry;

use crate::boundry::spawn_body;
use crate::sol::celestial_body::CelestialBody;
use crate::sol::reality_calulator::{calculate_new_positions, default_system};


use bevy::prelude::*;
// use bevy_mod_picking::prelude::*;
use gui::SolGuiPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum SimState {
    #[default]
    Running,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_state::<SimState>()
        // GUI
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
        .add_systems(Update, update_positions.run_if(in_state(SimState::Running)))
        .add_systems(Update, pause_on_space)
        // .add_systems(Update, calculate_collisions)
        .run();
}

fn big_bang(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("BANG");
    for body in default_system().clone() {
        spawn_body(body, &mut commands, &mut meshes, &mut materials);
    }
}

fn update_positions(time: Res<Time>, mut query: Query<(&mut CelestialBody, &mut Transform)>) {
    let bodies: Vec<CelestialBody> = query.iter().map(|(body, _)| body.clone()).collect();
    let cloned = bodies.clone();
    let new_positions = calculate_new_positions(&bodies, cloned);

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

fn pause_on_space(
    state: Res<State<SimState>>,
    mut next_state: ResMut<NextState<SimState>>,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<Input<KeyCode>>,
) {
    for (_, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }
        if input.just_pressed(KeyCode::Space) {
            let sim_state = state.get();
            match sim_state {
                SimState::Running => {
                    next_state.set(SimState::Paused);
                }
                SimState::Paused => {
                    next_state.set(SimState::Running);
                }
            }
        }
    }
}