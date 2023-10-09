mod camera;
mod gui;
mod sol;

use crate::gui::click_body;
use crate::sol::celestial_body::CelestialBody;
use crate::sol::reality_calulator::{calculate_new_positions, default_system};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use camera::SolCameraPlugin;
use gui::SolGuiPlugin;

use bevy_mod_picking::prelude::Click;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        // Camera Setup
        .add_plugins(SolCameraPlugin)
        // GUI
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(SolGuiPlugin)
        // Background
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 255.0,
        }))
        // Insert universe
        .add_systems(Startup, big_bang)
        // Systems to run every frame
        .add_systems(Update, update_positions)
        // .add_systems(Update, calculate_collisions)
        .run();
}

fn big_bang(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("BANG");
    for body in default_system().clone() {
        let transform = Transform::from_translation(Vec3::new(body.pos.x, body.pos.y, 0.));
        let radius = body.radius;
        let colour = body.get_surface_colour();

        commands.spawn((
            body,
            PickableBundle::default(),
            RaycastPickTarget::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(colour)),
                transform,
                ..default()
            },
            On::<Pointer<Click>>::run(click_body),
        ));
    }
}


fn update_positions(time: Res<Time>, mut query: Query<(&mut CelestialBody, &mut Transform)>) {
    let bodies: Vec<CelestialBody> = query.iter().map(|(body, _)| body.clone()).collect();
    let cloned = bodies.clone();
    let new_positions = calculate_new_positions(&bodies, cloned);

    for (mut body, mut transform) in query.iter_mut() {
        let new_body = new_positions
            .iter()
            .find(|x| x.name == body.name)
            .unwrap()
            .clone();

        body.pos = new_body.pos;
        body.momentum = new_body.momentum;
        transform.translation = Vec3::new(body.pos.x, body.pos.y, 0.);
    }
}
