use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_fly_camera::FlyCamera2d;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, zoom_2d);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        FlyCamera2d::default(),
        MainCamera,
    ));
}

fn zoom_2d(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut q: Query<&mut OrthographicProjection, With<FlyCamera2d>>,
) {
    for event in mouse_wheel_events.read() {
        let mut projection = q.single_mut();
        if event.y > 0.0 {
            // Scroll up
            projection.scale *= 0.95;
        } else if event.y < 0.0 {
            // Scroll down
            projection.scale *= 1.05;
        }
        // always ensure you end up with same values
        projection.scale = projection.scale.clamp(0.5, 800.0);
    }
}
