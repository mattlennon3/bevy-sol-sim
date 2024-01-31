use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_real_timer, transform::commands};

use super::{
    celestial_body::{
        celestial_body::{get_force_vector, ForceVectorData},
        Mass, Momentum, Position, TransformUpdates2d,
    },
    celestial_type::CelestialType,
};

// Change to 6.67e-11 for real world
pub const GRAVITY: f32 = 0.5;
pub const TIME_DELTA_PER_TICK: f64 = 0.005; // 0.016 is 60fps, but we need time to calculate things

#[derive(Component)]
pub struct Simulated;

#[derive(Component)]
pub struct MostMass;

pub struct RealityCalculatorPlugin;

impl Plugin for RealityCalculatorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimTime::default())
            .add_event::<StepForwardEvent>()
            .add_event::<StepBackwardEvent>()
            .add_systems(Startup, setup_time)
            .add_systems(PreUpdate, set_most_massive)
            .add_systems(Update, set_sim_time_flow)
            .add_systems(Update, update_positions);
    }
}

pub fn setup_time(mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(1.0);
}

// pub fn update_simulated_time_text()

#[derive(Clone, Debug)]
pub struct PositionParams {
    pub entity: Entity,
    pub transform: Transform,
    pub momentum: Momentum,
    pub mass: Mass,
}

#[derive(Resource)]
pub struct SimTime {
    sim_time: f64,
    flow_delta_time: Option<f64>,
    // sim_events: HashMap<f64, SimEvent>, // TODO: Not a hashmap, but some sort of ordered list
}

pub struct SimEvent {}

impl Default for SimTime {
    fn default() -> Self {
        Self {
            sim_time: 0.0,
            flow_delta_time: Some(0.0),
        }
    }
}

#[derive(Event)]
pub struct StepForwardEvent;
#[derive(Event)]
pub struct StepBackwardEvent;

pub type FlowDeltaTime = f32;
pub struct TimeError;

pub fn set_sim_time_flow(
    // Virtual time, helps with slomo and paused
    mut time: ResMut<Time<Virtual>>,
    // Sim time, as if I needed a 3rd time dimension
    mut sim_time: ResMut<SimTime>,
    mut step_forward: EventReader<StepForwardEvent>,
    mut step_backward: EventReader<StepBackwardEvent>,
) {
    let mut real_delta_time = time.delta_seconds_f64();

    if !time.is_paused() {
        if real_delta_time < TIME_DELTA_PER_TICK {
            sim_time.flow_delta_time = None;
        } else {
            sim_time.flow_delta_time = Some(real_delta_time);
            sim_time.sim_time += real_delta_time;
        }
        return;
    }

    // Advance time before doing calculations
    time.advance_by(Duration::from_secs_f64(TIME_DELTA_PER_TICK));
    // Re-fetch this, as we've just moved time forward
    real_delta_time = time.delta_seconds_f64();

    // STEP LOGIC
    let mut step = TIME_DELTA_PER_TICK;
    let mut flow: f64 = 1.0;

    if !step_forward.is_empty() {
        sim_time.sim_time += step;
    } else if !step_backward.is_empty() {
        flow = -1.0;

        // If the step would take the simtime below 0, then decrease it to match the step time
        if sim_time.sim_time - step < 0.0 {
            step = sim_time.sim_time;
        }

        if sim_time.sim_time <= 0.0 {
            info!("Can't step backward, already at 0 {:?}", sim_time.sim_time);
            sim_time.flow_delta_time = None;
            return;
        } else {
            sim_time.sim_time -= step;
        }
    } else {
        // else no path forward, return
        sim_time.flow_delta_time = None;
        step_forward.clear();
        step_backward.clear();
        return;
    }

    step_forward.clear();
    step_backward.clear();
    // Some time must advance even while stepping

    // INFO: I think it's right to use both these. As otherwise it
    // is a double negative at the end of this function (calculating mass, then the x,y)
    // UPDATE: I was wrong, all equations should use `flow_delta_time`
    let flow_delta_time = real_delta_time * flow;

    // Make sure this is a round number, or not crazy. To avoid floating point drift
    sim_time.flow_delta_time = Some(flow_delta_time);

    dbg!(
        sim_time.flow_delta_time,
        real_delta_time,
        time.delta_seconds()
    );
}

fn reset_step_events(
    mut step_forward: EventReader<StepForwardEvent>,
    mut step_backward: EventReader<StepBackwardEvent>,
) {
    step_forward.clear();
    step_backward.clear();
}

pub fn set_sim_time_flow_old(
    // Virtual time, helps with slomo and paused
    mut time: ResMut<Time<Virtual>>,
    // Sim time, as if I needed a 3rd time dimension
    mut sim_time: ResMut<SimTime>,
    mut step_forward: EventReader<StepForwardEvent>,
    mut step_backward: EventReader<StepBackwardEvent>,
) {
    let mut flow: f64 = 1.0;

    // TODO INSIDE HERE
    // Look ahead, if there is a SimTime event ahead, then slow the stepping to that point.
    // Take the action, then resume stepping at time delta pace

    // Refactor
    // Match function to get flow
    // Match function to get step
    // Match function to get TIME_DELTA_PER_TICK

    if time.is_paused() {
        let mut step = TIME_DELTA_PER_TICK as f64;
        if !step_forward.is_empty() {
            // if step forward event, set flow to 1, proceed
            step_forward.clear();
            // info!("Step forward");
            sim_time.sim_time += step;
            time.advance_by(Duration::from_secs_f64(step));
        } else if !step_backward.is_empty() {
            // if step backward event, set flow to -1, proceed
            step_backward.clear();
            flow = -1.0;

            // If the step would take the simtime below 0, then decrease it to match the step time
            if sim_time.sim_time - step < 0.0 {
                step = sim_time.sim_time;
            }

            if sim_time.sim_time <= 0.0 {
                info!("Can't step backward, already at 0 {:?}", sim_time.sim_time);
                sim_time.flow_delta_time = None;
            } else {
                sim_time.sim_time -= step;
            }
            // info!("Step backward");
            time.advance_by(Duration::from_secs_f64(step));
        } else {
            // else no path forward, return
            sim_time.flow_delta_time = None;
            return;
        }
    } else {
        // else check if we're running ahead of the min time delta tick

        /*
           Somewhat necessary, but perhaps scale down TIME_DELTA_PER_TICK if the
           time relative_speed gets lower than this constant.
           As the delta seconds will never get high enough
        */
        if time.delta_seconds_f64() < TIME_DELTA_PER_TICK {
            sim_time.flow_delta_time = None;
        } else {
            // Happy path, increment time, not by delta seconds, but by TIME_DELTA_PER_TICK
            sim_time.sim_time += time.delta_seconds_f64(); // time.delta_seconds_f64()
        }
    }

    // INFO: I think it's right to use both these. As otherwise it
    // is a double negative at the end of this function (calculating mass, then the x,y)
    // UPDATE: I was wrong, all equations should use `flow_delta_time`
    let real_delta_time = time.delta_seconds_f64();
    let flow_delta_time = real_delta_time * flow;

    // Make sure this is a round number, or not crazy. To avoid floating point drift
    sim_time.flow_delta_time = Some(flow_delta_time);
}

fn update_positions(
    mut query: Query<(Entity, &mut Transform, &mut Momentum, &mut Mass), With<Simulated>>,
    sim_time: ResMut<SimTime>,
    // step_forward: EventReader<StepForwardEvent>,
    // step_backward: EventReader<StepBackwardEvent>,
    // time: ResMut<Time<Virtual>>,
) {
    let Some(flow_delta_time) = sim_time.flow_delta_time else {
        return;
    };

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

        momentum.0 = momentum.0 + cumulitive_forces * flow_delta_time as f32;
        let translation = transform.translation;
        transform.update_position(Position(Vec2::new(
            translation.x + momentum.0.x / mass.0 * flow_delta_time as f32,
            translation.y + momentum.0.y / mass.0 * flow_delta_time as f32,
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
