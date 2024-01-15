use bevy::prelude::*;
use vector2d::Vector2D;

use crate::{
    boundry::spawn_body,
    gui::{
        camera::ui_camera::MainCamera,
        kb_mouse::mouse_states::{LeftClickActionState, UIMouseState},
    },
    sol::{celestial_body::CelestialBody, celestial_type::CelestialType},
};

// use super::follow_body::UIFollowBody;

#[derive(Resource, Debug)]
pub struct UIPlaceState {
    body_type: Option<CelestialType>,
}

impl Default for UIPlaceState {
    fn default() -> Self {
        // TODO Should be None
        Self {
            body_type: Some(CelestialType::PLANET),
        }
    }
}

pub fn spawn_selected_body_type(
    place_state: ResMut<UIPlaceState>,
    camera_q: Query<
        (&mut Camera, &mut GlobalTransform),
        With<MainCamera>, //With<RaycastPickable>,
    >,
    // follow_body: ResMut<UIFollowBody>,
    // query: Query<&CelestialBody>,
    mouse_input: Res<Input<MouseButton>>,
    mouse_state: Res<UIMouseState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    if mouse_state.left != LeftClickActionState::Spawning {
        return;
    }

    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(body_type) = place_state.body_type {
        let window = windows.single();
        if camera_q.is_empty() {
            return;
        }
        let (camera, camera_transform) = camera_q.single();
        // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
            let pos = Vector2D {
                x: world_position.x,
                y: world_position.y,
            };
            let default_momentum = Vector2D { x: 0.0, y: 0.0 };

            // If following a body, match it's momentum
            // TODO: This didn't work, bodies started pinging away on spawn...
            // let momentum = match follow_body.follow {
            //     Some(entity) => {
            //         if let Ok(body) = query.get(entity) {
            //             body.momentum
            //         } else {
            //             default_momentum
            //         }
            //     },
            //     None => default_momentum
            // };

            let body = CelestialBody::new(body_type, pos, 1.0, default_momentum);

            spawn_body(body, &mut commands, &mut meshes, &mut materials);
            // place_state.body_type = None;
        }
    }
}
