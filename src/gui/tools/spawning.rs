use bevy::{prelude::*, window::PrimaryWindow};
use vector2d::Vector2D;

use crate::{
    boundry::{create_celestial_body_mesh, Simulated},
    gui::{
        camera::ui_camera::MainCamera,
        constants::constants::Z_LEVELS,
        kb_mouse::mouse_states::{LeftClickActionState, UIMouseState},
    },
    sol::{celestial_body::CelestialBody, celestial_type::CelestialType},
};

#[derive(Component)]
pub struct SpawningBody;

#[derive(Resource, Debug)]
pub struct UIPlaceState {
    // pub body_type: Option<CelestialType>,
    // Populated when the user clicks and drags to spawn a body
    pub vec_start: Option<Vector2D<f32>>,
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
            .add_systems(Update, spawn_body)
            .add_systems(Update, spawn_spawning_body);
        // .add_systems(Update, render_click_and_drag_line);
    }
}

impl Default for UIPlaceState {
    fn default() -> Self {
        Self {
            // body_type: None,
            vec_start: None,
        }
    }
}

// Actually adds the components of the body, so it can be rendered
pub fn spawn_spawning_body(
    mut commands: Commands,
    mouse_state: Res<UIMouseState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<
        (&mut Transform, &CelestialType, Entity),
        (With<SpawningBody>, Without<CelestialBody>),
    >,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Check main query first
    if query.is_empty() {
        return;
    }

    if mouse_state.left != LeftClickActionState::Spawning {
        return;
    }

    // Check last as it involves calculations
    let Some(world_position) = get_world_cursor_position(q_windows, q_camera) else {
        return;
    };

    let (mut transform, body_type, entity) = query.single_mut();
    transform.translation = Vec3::new(world_position.x, world_position.y, Z_LEVELS.background);

    let body = CelestialBody::new_random(
        *body_type,
        // TODO: The transform is awkward here, we should be able to get the position from the transform
        // We need to update this later on spawn, as we are just following the cursor atm
        Vector2D {
            x: world_position.x,
            y: world_position.y,
        },
        // TODO: Set from Click + Drag
        Vector2D { x: 100., y: 100. },
    );
    let mesh = create_celestial_body_mesh(
        body.radius,
        body.get_surface_colour(),
        &mut meshes,
        &mut materials,
    );

    commands.entity(entity).insert((mesh, body));
}

// Keep following the cursor
pub fn spawning_body_follow_cursor(
    mouse_state: Res<UIMouseState>,
    mut q_transform: Query<&mut Transform, (With<SpawningBody>, Without<Simulated>)>,
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

// Begin selection
pub fn start_spawn_selection(
    mut commands: Commands,
    mut mouse_state: ResMut<UIMouseState>,
    mut start_spawning: EventReader<StartSpawningEvent>,
) {
    if let Some(body_type) = start_spawning.read().last() {
        info!("Selected for spawn: {:?}", body_type.0);
        mouse_state.left = LeftClickActionState::Spawning;
        // Only spawn this component, then detect later and add the full mesh when we know the cursor position
        commands.spawn((
            // unwrap event and get body type
            body_type.0,
            SpawningBody,
            Transform::default(),
        ));
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

// Remove the SpawningBody component and add the Simulated component
pub fn spawn_body(
    mut commands: Commands,
    mut mouse_state: ResMut<UIMouseState>,
    mut place_state: ResMut<UIPlaceState>,
    mouse: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_body: Query<(Entity, &mut CelestialBody), With<SpawningBody>>,
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
        place_state.vec_start = Some(Vector2D {
            x: world_position.x,
            y: world_position.y,
        });
    }

    if mouse.just_released(MouseButton::Left) {
        if let Ok((entity, mut body)) = q_body.get_single_mut() {
            let Some(vec_start) = place_state.vec_start else {
                return;
            };
            // Update position of body, as it is set within the CelestialBody struct
            let momentum_multiplier = 5.0;
            body.momentum = Vector2D { x: (vec_start.x + body.pos.x) * momentum_multiplier, y: (vec_start.y + body.pos.y) * momentum_multiplier};
            body.pos = Vector2D {
                x: world_position.x,
                y: world_position.y,
            };
            commands.entity(entity).remove::<SpawningBody>();
            commands.entity(entity).insert(Simulated);
            info!("Spawned a {:?}, named: {:?}. Pos: {:?}, Momentum: {:?}", body.body_type, body.name, body.pos, body.momentum);
            mouse_state.left = LeftClickActionState::Selecting;
            place_state.vec_start = None;
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
    let Some(world_position) = get_world_cursor_position(q_windows, q_camera) else {
        return;
    };

    if let Some(vec_start) = place_state.vec_start {
        let bevy_vec2 = bevy::prelude::Vec2::new(vec_start.x, vec_start.y);
        gizmos.line_2d(bevy_vec2, world_position, Color::WHITE);
    }
}

// fn render_click_and_drag_line(mut cursor_evr: EventReader<CursorMoved>) {
//     if let Some(position) = cursor_evr.read().last() {
//         println!("Cursor has moved to {:?}", position);
//     }
// }

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
