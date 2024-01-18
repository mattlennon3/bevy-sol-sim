use std::f32::consts::PI;

use bevy::prelude::*;

use crate::sol::{
    celestial_body::CelestialBody,
    reality_calulator::{get_distance, get_object_with_most_mass},
};

pub struct TrajectoryPlugin;

impl Plugin for TrajectoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_perfect_orbit)
            .add_systems(Update, render_trajectory);
    }
}

#[derive(Component)]
pub struct ShowTrajectory;

fn render_trajectory(b_query: Query<&CelestialBody, With<ShowTrajectory>>) {
    // let bodies: Vec<CelestialBody> = b_query.iter().cloned().collect();
    // let cloned = bodies.clone();
    // let new_positions = calculate_new_positions(&bodies, cloned);

    // let most_mass = get_object_with_most_mass(&bodies);

    // draw a circle between each body and the star

    // linestrip_gradient_2d(
    //     &mut gizmos,
    //     &new_positions,
    //     most_mass,
    //     Color::WHITE,
    //     Color::BLACK,
    // );
}

#[derive(Component)]
pub struct ShowBasicOrbit;

fn draw_perfect_orbit(b_query: Query<&CelestialBody, With<ShowBasicOrbit>>, mut gizmos: Gizmos) {
    let bodies: Vec<CelestialBody> = b_query.iter().cloned().collect();
    if bodies.is_empty() {
        return;
    }
    let most_mass = get_object_with_most_mass(&bodies);

    for body in &bodies {
        if body != most_mass {
            let distance = get_distance(&body, &most_mass);
            let radius = distance;
            let center = Vec2::new(most_mass.pos.x, most_mass.pos.y);
            // println!(" {} {} {}", distance, radius, center);
            gizmos
                .arc_2d(center, 0., PI * 2.0, radius, Color::GRAY)
                .segments(128);
        }
    }
}
