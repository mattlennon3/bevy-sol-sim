use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    gui::camera::ui_camera::MainCamera,
    sol::celestial_type::CelestialType,
};

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
    query: Query<&CelestialType>,
    event: Listener<Pointer<Click>>,
) {
    if let Ok(_body) = query.get(event.target) {
        active_body.follow = Some(event.target.clone());
    }
}

pub fn follow_body(
    mut follow_body: ResMut<UIFollowBody>,
    query: Query<(Entity, &CelestialType, &Transform), Without<MainCamera>>,
    mut camera_transform: Query<&mut Transform, With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    match follow_body.follow {
        Some(followed) => {
            if let Ok((_, body_type, transform)) = query.get(followed) {
                // set camera transform to body position
                if let Ok(mut camera_transform) = camera_transform.get_single_mut() {
                    camera_transform.translation = transform.translation;
                }
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
