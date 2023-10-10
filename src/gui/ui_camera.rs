use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy_fly_camera::FlyCamera2d;
use bevy_mod_picking::prelude::RaycastPickCamera;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(RaycastPickCamera::default())
        .insert(FlyCamera2d::default());
}

pub fn zoom_2d(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut q: Query<&mut OrthographicProjection, With<FlyCamera2d>>,
) {
    for event in mouse_wheel_events.iter() {
        let mut projection = q.single_mut();
        if event.y > 0.0 {
            // Scroll up
            projection.scale *= 0.95;
        } else if event.y < 0.0 {
            // Scroll down
            projection.scale *= 1.05;
        }
        // always ensure you end up with sane values
        projection.scale = projection.scale.clamp(0.5, 5.0);
    }
}
