use bevy::prelude::*;
use bevy::prelude::Time;
use std::time::Duration;

#[derive(Resource, Debug, Copy, Clone)]
pub struct SimTime(u64);

impl Default for SimTime {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Resource, Debug, Default, States, Hash, Eq, PartialEq, Clone, Copy)]
pub enum TimeAction {
    #[default]
    Forward,
    Rewind,
    Paused,
}

#[derive(Resource, Debug)]
pub struct TimeState {
    state: TimeAction,
    previous_state: TimeAction,
    /** Used to 0.5x, x2 or 10x the time speed */
    time_multiplier: u16,
    step_multiplier: u16,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            state: TimeAction::Forward,
            previous_state: TimeAction::Forward,
            time_multiplier: 1,
            step_multiplier: 1,
        }
    }
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimTime::default())
            .insert_resource(TimeState::default())
            .add_state::<TimeAction>()
            .add_systems(Update, time_system)
            .add_systems(Update, pause_on_space)
            .add_systems(Update, step_keybinds);
    }
}

/// `Virtual` time related marker
#[derive(Component)]
struct VirtualTime;

pub fn time_system(
    mut time: ResMut<SimTime>,
    mut actual_time: Res<Time<Virtual>>,
    time_state: ResMut<TimeState>,
) {

    // TODO: Lock so 1 actual second === 1 time second by using )
    // Then we can use the time multiplier to speed up or slow down time
    actual_time.delta_seconds();
    actual_time.relative_speed();
    actual_time.set_relative_speed(ratio);


    match time_state.state {
        TimeAction::Forward => {
            time.0 += 1 * time_state.time_multiplier as u64;
        }
        TimeAction::Rewind => {
            time.0 -= 1 * time_state.time_multiplier as u64;
        }
        TimeAction::Paused => (),
    }
}

fn step_keybinds(
    mut time: ResMut<SimTime>,
    mut time_state: ResMut<TimeState>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Left) {
        time.0 += 1 * time_state.time_multiplier as u64;
    }
    if input.just_pressed(KeyCode::Right) {
        time.0 -= 1 * time_state.time_multiplier as u64;
    }

    if input.just_pressed(KeyCode::Up) {
        time_state.time_multiplier = time_state.time_multiplier + 10;
    }
    if input.just_pressed(KeyCode::Down) {
        time_state.time_multiplier = time_state.time_multiplier - 10;
    }
}

fn pause_on_space(
    state: Res<State<TimeAction>>,
    mut next_state: ResMut<NextState<TimeAction>>,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<Input<KeyCode>>,
) {
    for (_, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }
        if input.just_pressed(KeyCode::Space) {
            let time_state = state.get();
            match time_state {
                TimeAction::Paused => {
                    next_state.set(TimeAction::Forward);
                }
                _ => (),
            }
        }
    }
}
