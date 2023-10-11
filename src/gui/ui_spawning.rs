use bevy::{input::mouse, prelude::*};
use bevy_mod_picking::prelude::*;
use vector2d::Vector2D;

use crate::{
    boundry::spawn_body,
    sol::{celestial_body::CelestialBody, celestial_type::CelestialType},
};

use super::ui_camera::MainCamera;

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
    mut place_state: ResMut<UIPlaceState>,
    camera_q: Query<
        (&mut Camera, &mut GlobalTransform),
        (With<RaycastPickCamera>, With<MainCamera>),
    >,
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    if let Some(body_type) = place_state.body_type {
        if mouse_input.just_pressed(MouseButton::Left) {
            let window = windows.single();
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
                let momentum = Vector2D { x: 0.0, y: 0.0 };
                let body = CelestialBody::new(body_type, pos, 1.0, momentum);
                info!("Spawning {:?}", body);

                spawn_body(body, &mut commands, &mut meshes, &mut materials);

                place_state.body_type = None;
            }
        }
    }
}
