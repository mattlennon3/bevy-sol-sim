use bevy::{prelude::*, transform::commands};

use crate::gui::panels::ui_time::TimeState;

use super::{
    celestial_body::{
        celestial_body::{get_force_vector, ForceVectorData},
        Mass, Momentum, Position, TransformUpdates2d,
    },
    celestial_type::CelestialType,
};

// Change to 6.67e-11 for real world
pub const GRAVITY: f32 = 0.5;
// pub const TIME_START: f32 = 0.0;
pub const TIME_DELTA_PER_TICK: f32 = 0.005;

// pub fn get_trajectory_path(body: &CelestialBody, system: SystemContents) -> Vec<Vec2<f32>> {
//     // return body.trail.clone();
// }

#[derive(Component)]
pub struct Simulated;

#[derive(Component)]
pub struct MostMass;

pub struct RealityCalculatorPlugin;

impl Plugin for RealityCalculatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, set_most_massive)
            .add_systems(Update, update_positions);
    }
}

#[derive(Clone, Debug)]
pub struct PositionParams {
    pub entity: Entity,
    pub transform: Transform,
    pub momentum: Momentum,
    pub mass: Mass,
}

fn update_positions(
    mut query: Query<(Entity, &mut Transform, &mut Momentum, &mut Mass), With<Simulated>>,
) {
    // Make a copy of the current state of the query
    let bodies: Vec<(Entity, Transform, Momentum, Mass)> = query
        .iter()
        .map(|(entity, transform, momentum, mass)| (entity, *transform, *momentum, *mass))
        .collect();

    // Mutably iterate over the query
    for (entity, mut transform, mut momentum, mass) in query.iter_mut() {
        let mut forces: Vec<Vec2> = vec![];

        for (other_entity, other_transform, other_momentum, other_mass) in &bodies {
            if other_entity != &entity {
                let current = ForceVectorData {
                    pos: Position::new_from_transform(&transform),
                    mass: *mass,
                    momentum: *momentum,
                };
                // println!("!!!");
                // dbg!(&transform.translation, current.pos);
                let other = ForceVectorData {
                    pos: Position::new_from_transform(&other_transform),
                    mass: *other_mass,
                    momentum: *other_momentum,
                };
                // dbg!(&other_transform.translation, other.pos);

                forces.push(get_force_vector(&current, &other));
            }
        }
        let cumulitive_forces: Vec2 = forces
            .iter()
            .fold(Vec2 { x: 0.0, y: 0.0 }, |acc, x| acc + *x);

        momentum.0 = momentum.0 + cumulitive_forces * TIME_DELTA_PER_TICK;
        let translation = transform.translation;
        transform.update_position(Position(Vec2::new(
            translation.x + momentum.0.x / mass.0 * TIME_DELTA_PER_TICK,
            translation.y + momentum.0.y / mass.0 * TIME_DELTA_PER_TICK,
        )));
    }

    // if (bodies.len() == 4) {
    //     dbg!(bodies);
    //     panic!("bodies");
    // }
}

fn set_most_massive(
    q_all: Query<(Entity, &Mass), (With<Simulated>, With<CelestialType>)>,
    mut q_massive: Query<Entity, (With<Simulated>, With<MostMass>, With<CelestialType>)>,
    mut commands: Commands,
) {
    let most_massive = q_all
        .iter()
        .reduce(|acc, mass| if acc.1 .0 >= mass.1 .0 { acc } else { mass })
        .unwrap();
    if let Ok(entity) = q_massive.get_single_mut() {
        if entity == most_massive.0 {
            // Already set, return
            return;
        } else {
            commands.entity(entity).remove::<MostMass>();
        }
    }
    commands.entity(most_massive.0).insert(MostMass);
}
