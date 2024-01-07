use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::backends::raycast::RaycastPickable;
use bevy_mod_picking::prelude::*;

use crate::gui::assets::asset_loader::SceneAssets;
use crate::sol::celestial_body::CelestialBody;
use crate::gui::panels::ui_selected_body::describe_body;
use crate::gui::tools::follow_body::click_body;

// TODO:
// READ THIS
// https://bevy-cheatbook.github.io/programming/bundle.html#loose-components-as-bundles

// Bundles vs tuples


#[derive(Bundle)]
struct CelestialBodyBundle {
    body: CelestialBody,
    clicked: PickableBundle,
    over: PickableBundle,
    // raycast: RaycastPickable,
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

// impl Plugin for CelestialBodyBundle {
//     fn build(&self, app: &mut App) {

//     }
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
    let mesh = create_celestial_body_mesh(body.radius, body.get_surface_colour(), meshes, materials);

    info!("Spawning {:?}", &body);
    commands.spawn((
        body,
        PickableBundle::default(),
        RaycastPickable,
        mesh,
        On::<Pointer<Click>>::run(click_body),
        On::<Pointer<Over>>::run(describe_body),
    ));
}

// TODO: Change this to a celestialtype and some other props, so I don't need to configure a whole body for the bottom panel?
// pub fn create_celestial_body_mesh (body: &CelestialBody, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> MaterialMesh2dBundle<ColorMaterial> {
//     let radius = body.radius;
//     let colour = body.get_surface_colour();

//     MaterialMesh2dBundle {
//         mesh: meshes.add(shape::Circle::new(radius).into()).into(),
//         material: materials.add(ColorMaterial::from(colour)),
//         transform,
//         ..default()
//     }
// }

pub fn create_celestial_body_mesh(radius: f32, colour: Color, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> MaterialMesh2dBundle<ColorMaterial> {
    // let transform = Transform::from_translation(Vec3::new(body.pos.x, body.pos.y, 0.));

    MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(radius).into()).into(),
        material: materials.add(ColorMaterial::from(colour)),
        // transform,
        ..default()
    }
}
pub fn create_celestial_body_scene(radius: f32, scene_assets: Res<SceneAssets>) -> SceneBundle {
    // let transform = Transform::from_translation(Vec3::new(body.pos.x, body.pos.y, 0.));

    SceneBundle {
        scene: scene_assets.star.clone(),
        // transform: Transform::from_translation(translation),
        ..default()
    }
}

