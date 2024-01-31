use bevy::prelude::Time;
use bevy::prelude::*;
use crate::sol::reality_calculator::{
    StepBackwardEvent, StepForwardEvent, TIME_DELTA_PER_TICK,
};

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_on_space)
            .add_systems(Update, speed_keybinds)
            .add_systems(Update, step_keybinds);
    }
}

const MAX_TIME_MULTIPLIER: f32 = 10.0;

fn speed_keybinds(mut time: ResMut<Time<Virtual>>, input: Res<Input<KeyCode>>) {
    let delta = 0.5;
    let current_speed = time.relative_speed();

    if input.just_pressed(KeyCode::Up) {
        time.set_relative_speed(
            (current_speed + delta).clamp(TIME_DELTA_PER_TICK as f32 + 0.001, MAX_TIME_MULTIPLIER),
        );
    }
    if input.just_pressed(KeyCode::Down) {
        time.set_relative_speed(
            (current_speed - delta).clamp(TIME_DELTA_PER_TICK as f32 + 0.001, MAX_TIME_MULTIPLIER),
        );
    }
}

fn step_keybinds(
    time: Res<Time<Virtual>>,
    input: Res<Input<KeyCode>>,
    mut step_forward: EventWriter<StepForwardEvent>,
    mut step_backward: EventWriter<StepBackwardEvent>,
) {
    // Only allow stepping when paused
    if !time.is_paused() {
        return;
    }

    // INFO: Can be changed to just_pressed for incremental stepping
    // TODO: Add a "wait" before holding the button to step rapidly
    if input.pressed(KeyCode::Left) {
        step_backward.send(StepBackwardEvent);
    }
    if input.pressed(KeyCode::Right) {
        step_forward.send(StepForwardEvent);
    }
}

fn pause_on_space(input: Res<Input<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if input.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            info!("UNPAUSED!");
            time.unpause();
        } else {
            info!("PAUSED!");
            time.pause();
        }
    }
}
