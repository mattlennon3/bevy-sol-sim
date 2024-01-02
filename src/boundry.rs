use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;

use crate::sol::celestial_body::CelestialBody;
use crate::gui::ui_selected_body::describe_body;
use crate::gui::ui_follow_body::click_body;

// TODO:
// READ THIS
// https://bevy-cheatbook.github.io/programming/bundle.html#loose-components-as-bundles

// Bundles vs tuples


// #[derive(Bundle)]
// struct CelestialBodyBundle {
//     body: CelestialBody,
//     clicked: PickableBundle,
//     over: PickableBundle,
//     raycast: RaycastPickTarget,
//     material_mesh: MaterialMesh2dBundle<ColorMaterial>,
// }

// impl Default for CelestialBodyBundle {
//     fn default() -> Self {
//         Self {
//             body: CelestialBody::default(),
//             pickable: PickableBundle::default(),
//             raycast: RaycastPickTarget::default(),
//             material_mesh: MaterialMesh2dBundle {
//                 mesh: Mesh::from(shape::Circle::new(1.0)),
//                 material: ColorMaterial::from(Color::WHITE),
//                 ..Default::default()
//             },
//         }
//     }
// }

pub fn spawn_body (body: CelestialBody, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let transform = Transform::from_translation(Vec3::new(body.pos.x, body.pos.y, 0.));
    let radius = body.radius;
    let colour = body.get_surface_colour();

    info!("Spawning {:?}", body);
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
        On::<Pointer<Over>>::run(describe_body),
    ));
}
