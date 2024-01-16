use bevy::{
    ecs::query,
    input::mouse::MouseMotion,
    prelude::*,
    transform::commands,
    window::{Cursor, PrimaryWindow},
};
use vector2d::Vector2D;

use crate::{
    boundry::{create_celestial_body_mesh, Simulated},
    gui::{
        camera::ui_camera::MainCamera,
        kb_mouse::mouse_states::{LeftClickActionState, UIMouseState}, constants::constants::Z_LEVELS,
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

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIPlaceState>()
            .add_systems(Update, update_spawning_body_at_cursor)
            .add_systems(Update, add_spawning_body_to_cursor);
        // .add_systems(Update, render_click_and_drag_line);
        // .add_systems(Update, spawn_selected_body_type);
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

pub fn add_spawning_body_to_cursor(
    // place_state: ResMut<UIPlaceState>,
    mouse_state: Res<UIMouseState>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<
        (&mut Transform, &CelestialType, Entity),
        (With<SpawningBody>, Without<CelestialBody>),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: The mouse state is never set to Spawning - so this never runs
    // I need to figure out what the relation between this mouse state + the spawn_spawning_body fn is
    if mouse_state.left != LeftClickActionState::Spawning {
        return;
    }

    // Get mouse position if it's on screen
    let Some(position) = q_windows.single().cursor_position() else {
        return;
    };

    if query.is_empty() {
        return;
    }

    println!("add_spawning_body_to_cursor");

    let (mut transform, body_type, entity) = query.single_mut();
    transform.translation = Vec3::new(position.x, position.y, 0.0);

    let body = CelestialBody::new_random(
        *body_type,
        Vector2D {
            x: position.x,
            y: position.y,
        },
        Vector2D { x: 0., y: 0. },
    );
    let mesh = create_celestial_body_mesh(
        body.radius,
        body.get_surface_colour(),
        &mut meshes,
        &mut materials,
    );

    commands.entity(entity).insert((mesh, body));

    // place_state.is_added()
    // if Some(body) = place_state.body_type {
    //     commands.spawn((
    //         SpawningBody,
    //         Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //         GlobalTransform::default(),
    //         body,
    //     ));
    // }
}


pub fn update_spawning_body_at_cursor(
    place_state: ResMut<UIPlaceState>,
    mouse_state: Res<UIMouseState>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, (With<SpawningBody>, Without<Simulated>)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // try rendering a body here, the absence of the Simulated struct will prevent it from being simulated (hopefully)

    if mouse_state.left != LeftClickActionState::Spawning {
        return;
    }

    if query.is_empty() {
        return;
    }

    if let Some(world_position) = get_world_cursor_position(q_windows, q_camera) {
        let mut transform = query.single_mut();
        // update body position to be cursor position
        transform.translation = Vec3::new(world_position.x, world_position.y, Z_LEVELS.background);
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
    
    // Get mouse position if it's on screen
    // let Some(position) = q_windows.single().cursor_position() else {
    //     return;
    // };
}

fn render_click_and_drag_line(mut cursor_evr: EventReader<CursorMoved>) {
    if let Some(position) = cursor_evr.read().last() {
        println!("Cursor has moved to {:?}", position);
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

pub fn spawn_spawning_body(
    body_type: CelestialType,
    mouse_state: &mut ResMut<UIMouseState>,
    // meshes: &mut ResMut<Assets<Mesh>>,
    //     materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    // let body = CelestialBody::new_random(body_type, pos, momentum);
    // let mesh = create_celestial_body_mesh(body.radius, body.get_surface_colour(), meshes, materials);

    mouse_state.left = LeftClickActionState::Spawning;
    println!("Spawn {}", body_type);
    // Only spawn this component, then detect later and add the full mesh when we know the cursor position
    commands.spawn((
        body_type,
        SpawningBody,
        // Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Transform::default(), // GlobalTransform::default(),
                              // mesh,
    ));
}

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