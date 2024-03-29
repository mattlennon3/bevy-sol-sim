use bevy::{prelude::*, transform, window::PrimaryWindow};

use crate::{
    gui::{
        camera::ui_camera::MainCamera, constants::constants::Z_LEVELS, kb_mouse::mouse_states::{LeftClickActionState, UIMouseState}, sol_gui::celestial_body_gui::CelestialBodyGuiBundle
    },
    sol::{
        celestial_body::{celestial_body::calculate_orbital_momentum, BodyName, CelestialBodyBundle, Mass, Momentum, Position},
        celestial_type::CelestialType,
        reality_calculator::{MostMass, Simulated},
    },
};

use super::trajectory::ShowBasicOrbit;

#[derive(Component)]
pub struct SpawningBody;

#[derive(Resource, Debug)]
pub struct UIPlaceState {
    // pub body_type: Option<CelestialType>,
    // Populated when the user clicks and drags to spawn a body
    pub click_drag_vec_start: Option<Position>,
    trajectory_mode: TrajectoryPresetType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TrajectoryPresetType {
    Orbital,
    ClickAndDrag,
}

pub struct SpawningPlugin;

// TODO's
/**
 * Click and drag to add momentum to a body
 *
 * Orbit view
 *
 */

#[derive(Event)]
pub struct StartSpawningEvent(pub CelestialType);

#[derive(Event)]
pub struct EndSpawningEvent;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIPlaceState>()
            .add_event::<StartSpawningEvent>()
            .add_event::<EndSpawningEvent>()
            .add_systems(Update, render_click_and_drag_line)
            .add_systems(Update, spawning_body_follow_cursor)
            .add_systems(Update, start_spawn_selection)
            .add_systems(Update, remove_spawn_selection)
            .add_systems(Update, spawn_body);
        // .add_systems(Update, render_click_and_drag_line);
    }
}

impl Default for UIPlaceState {
    fn default() -> Self {
        Self {
            click_drag_vec_start: None,
            trajectory_mode: TrajectoryPresetType::Orbital,
        }
    }
}

// Begin selection
pub fn start_spawn_selection(
    mut commands: Commands,
    mut mouse_state: ResMut<UIMouseState>,
    mut start_spawning: EventReader<StartSpawningEvent>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Check last as it involves calculations
    let Some(world_position) = get_world_cursor_position(q_windows, q_camera) else {
        return;
    };

    if let Some(spawn_body_type) = start_spawning.read().last() {
        info!("Selected for spawn: {:?}", spawn_body_type.0);
        mouse_state.left = LeftClickActionState::Spawning;
        // Only spawn this component, then detect later and add the full mesh when we know the cursor position

        let body = CelestialBodyBundle::new(
            spawn_body_type.0,
            Position::new(world_position.x, world_position.y),
            None, // random mass
        );

        commands.spawn(body).insert((SpawningBody, ShowBasicOrbit));
    }
}

// pub fn log_spawning_body(q: Query<(Entity, &CelestialType, &Transform, &Name), With<SpawningBody>>) {
//     if let Ok(entity) = q.get_single() {
//         info!("Spawning body: {:?}", entity);
//     }
// }

// Remove the SpawningBody component and add the Simulated component
pub fn spawn_body(
    mut commands: Commands,
    mut mouse_state: ResMut<UIMouseState>,
    mut place_state: ResMut<UIPlaceState>,
    mouse: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_most_mass: Query<(&Transform, &Mass, &Momentum), With<MostMass>>,
    mut q_body: Query<
        (Entity, &CelestialType, &Transform, &BodyName, &Mass),
        With<SpawningBody>,
    >,
) {
    if mouse_state.left != LeftClickActionState::Spawning {
        return;
    }

    // Check last as it involves calculations
    let Some(world_position) = get_world_cursor_position(q_windows, q_camera) else {
        return;
    };

    // Check for mouse releases
    if mouse.just_pressed(MouseButton::Left) {
        place_state.click_drag_vec_start = Some(Position::new(world_position.x, world_position.y));
    }

    if mouse.just_released(MouseButton::Left) {
        if let Ok((entity, body_type, pos, name, mass)) = q_body.get_single_mut() {
            // TODO: Need to calculate the orbital velocity based on the distance from the star and mass
            let Some(vec_start) = place_state.click_drag_vec_start else {
                return;
            };
            let Ok(most_mass) = q_most_mass.get_single() else {
                return;
            };
            
            // let momentum_multiplier = 5.0;
            // let momentum = Momentum::new(
            //     (vec_start.0.x + pos.translation.x) * momentum_multiplier,
            //     (vec_start.0.y + pos.translation.y) * momentum_multiplier,
            // );
            let momentum = calculate_orbital_momentum(most_mass.into(), (Position(world_position), mass), false);
            commands.entity(entity).remove::<SpawningBody>();
            commands.entity(entity).remove::<ShowBasicOrbit>();
            commands.entity(entity).insert(momentum);
            commands.entity(entity).insert(Simulated); 
            commands.entity(entity).insert(CelestialBodyGuiBundle::new());
            info!(
                "Spawned a {:?}, named: {:?}. Pos: {:?}, Momentum: {:?}",
                body_type, name, pos, momentum.0
            );
            mouse_state.left = LeftClickActionState::Selecting;
            place_state.click_drag_vec_start = None;
        }
    }
}

fn render_click_and_drag_line(
    mut mouse_state: ResMut<UIMouseState>,
    mut place_state: ResMut<UIPlaceState>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut gizmos: Gizmos,
) {
    if (place_state.trajectory_mode != TrajectoryPresetType::ClickAndDrag) {
        return;
    }

    let Some(world_position) = get_world_cursor_position(q_windows, q_camera) else {
        return;
    };

    if let Some(vec_start) = place_state.click_drag_vec_start {
        gizmos.line_2d(vec_start.0, world_position, Color::WHITE);
    }
}

// fn render_click_and_drag_line(mut cursor_evr: EventReader<CursorMoved>) {
//     if let Some(position) = cursor_evr.read().last() {
//         println!("Cursor has moved to {:?}", position);
//     }
// }

// Keep following the cursor
pub fn spawning_body_follow_cursor(
    mouse_state: Res<UIMouseState>,
    mut q_transform: Query<&mut Transform, With<SpawningBody>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if q_transform.is_empty() {
        return;
    }

    if mouse_state.left != LeftClickActionState::Spawning {
        return;
    }

    if let Some(world_position) = get_world_cursor_position(q_windows, q_camera) {
        let mut transform = q_transform.single_mut();
        // update body position to be cursor position
        transform.translation = Vec3::new(world_position.x, world_position.y, Z_LEVELS.background);
    }
}

// Cancel selection
pub fn remove_spawn_selection(
    mut commands: Commands,
    mut mouse_state: ResMut<UIMouseState>,
    mut end_spawning: EventReader<EndSpawningEvent>,
    query: Query<Entity, With<SpawningBody>>,
) {
    if let Some(_) = end_spawning.read().last() {
        mouse_state.left = LeftClickActionState::Selecting;
        if let Ok(entity) = query.get_single() {
            info!("Cancel spawning body");
            commands.entity(entity).despawn_recursive();
        }
    }
}

// pub fn spawn_selected_body_type(
//     place_state: ResMut<UIPlaceState>,
//     camera_q: Query<
//         (&mut Camera, &mut GlobalTransform),
//         With<MainCamera>, //With<RaycastPickable>,
//     >,
//     // follow_body: ResMut<UIFollowBody>,
//     // query: Query<&CelestialBody>,
//     mouse_input: Res<Input<MouseButton>>,
//     mouse_state: Res<UIMouseState>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     windows: Query<&Window>,
// ) {
//     if mouse_state.left != LeftClickActionState::Spawning {
//         return;
//     }

//     // mouse_input.just_released(input)
//     if !mouse_input.just_pressed(MouseButton::Left) {
//         return;
//     }

//     // let pos = Vector2D { x: 240.0, y: 0.0 };
//     // let momentum = Vector2D { x: 0.0, y: 100.0 };
//     // let body = CelestialBody::new_random(CelestialType::ASTEROID, pos, momentum);

//     if let Some(body_type) = place_state.body_type {
//         let window = windows.single();
//         if camera_q.is_empty() {
//             return;
//         }
//         let (camera, camera_transform) = camera_q.single();
//         // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
//         if let Some(world_position) = window
//             .cursor_position()
//             .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
//         {
//             // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
//             let pos = Vector2D {
//                 x: world_position.x,
//                 y: world_position.y,
//             };
//             let default_momentum = Vector2D { x: 0.0, y: 0.0 };

//             // If following a body, match it's momentum
//             // TODO: This didn't work, bodies started pinging away on spawn...
//             // let momentum = match follow_body.follow {
//             //     Some(entity) => {
//             //         if let Ok(body) = query.get(entity) {
//             //             body.momentum
//             //         } else {
//             //             default_momentum
//             //         }
//             //     },
//             //     None => default_momentum
//             // };

//             let body = CelestialBody::new(body_type, pos, 1.0, default_momentum);

//             spawn_body(body, &mut commands, &mut meshes, &mut materials);
//             // place_state.body_type = None;
//         }
//     }
// }

/**
 *
 * ##################### Non-Systems #############################
 *
 */

pub fn get_world_cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();
    let window = q_windows.single();

    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}
