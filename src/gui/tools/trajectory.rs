use std::f32::consts::PI;

use bevy::{prelude::*, transform};

use crate::sol::{
    celestial_body::{Mass, Position},
    celestial_type::CelestialType,
    reality_calculator::Simulated,
    utils::{get_distance, get_object_with_most_mass},
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

fn render_trajectory(b_query: Query<&Transform, With<ShowTrajectory>>) {
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

struct TrajectoryProps {
    mass: Mass,
    pos: Position,
}

fn draw_perfect_orbit(
    q_all: Query<(&CelestialType, &Mass, &Transform), With<Simulated>>,
    q_target: Query<&Transform, With<ShowBasicOrbit>>,
    mut gizmos: Gizmos,
) {
    let transforms = q_target.iter().collect::<Vec<_>>();
    if transforms.is_empty() {
        return;
    }
    let bodies: Vec<TrajectoryProps> = q_all
        .iter()
        .map(|(_, mass, transform)| TrajectoryProps {
            mass: *mass,
            pos: Position::new(transform.translation.x, transform.translation.y),
        })
        .collect();
    if bodies.is_empty() {
        return;
    }
    let most_mass_body = bodies.iter().max_by(|a, b| a.mass.partial_cmp(&b.mass).unwrap()).unwrap();

    for transform in &transforms {
        let distance = get_distance(&most_mass_body.pos, &Position::new(transform.translation.x, transform.translation.y));
        let radius = distance;
        let center = Position::new(most_mass_body.pos.0.x, most_mass_body.pos.0.y);
        gizmos
            .arc_2d(center.0, 0., PI * 2.0, radius.0, Color::GRAY)
            .segments(128);
    }
}