use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_mod_picking::{prelude::*, backends::raycast::RaycastPickable};

use crate::{sol::celestial_body::CelestialBody, gui::camera::ui_camera::MainCamera};

#[derive(Resource, Debug)]
pub struct UIFollowBody {
    pub follow: Option<Entity>,
}

impl Default for UIFollowBody {
    fn default() -> Self {
        Self { follow: None }
    }
}

pub fn click_body(
    mut active_body: ResMut<UIFollowBody>,
    query: Query<&CelestialBody>,
    event: Listener<Pointer<Click>>,
) {
    if let Ok(_body) = query.get(event.target) {
        active_body.follow = Some(event.target.clone());
    }
}

pub fn follow_body(
    mut follow_body: ResMut<UIFollowBody>,
    mut egui_contexts: EguiContexts,
    query: Query<&CelestialBody>,
    mut q: Query<&mut Transform, With<RaycastPickable>>, // TODO remove  RaycastPickable and fix this fn
    camera_q: Query<
        (&mut Camera, &mut GlobalTransform),
        With<MainCamera>, //With<RaycastPickable>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
) {
    match follow_body.follow {
        Some(followed) => {
            if let Ok(body) = query.get(followed) {
              q.for_each_mut(|mut transform| {
                  transform.translation = Vec3 {
                      x: body.pos.x,
                      y: body.pos.y,
                      z: 0.0,
                  };
              });
            }
            if keyboard_input.just_pressed(KeyCode::W) {
              follow_body.follow = None;
            }
            if keyboard_input.just_pressed(KeyCode::A) {
                follow_body.follow = None;
            }
            if keyboard_input.just_pressed(KeyCode::S) {
                follow_body.follow = None;
            }
            if keyboard_input.just_pressed(KeyCode::D) {
                follow_body.follow = None;
            }
        }
        None => (),
    }
}
