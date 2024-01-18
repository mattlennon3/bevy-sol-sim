use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::backends::raycast::RaycastPickable;
use bevy_mod_picking::prelude::*;

use crate::gui::assets::asset_loader::SceneAssets;
use crate::gui::panels::ui_selected_body::describe_body;
use crate::gui::tools::follow_body::click_body;
use crate::sol::celestial_body::CelestialBody;

// TODO:
// READ THIS
// https://bevy-cheatbook.github.io/programming/bundle.html#loose-components-as-bundles

// Bundles vs tuples

#[derive(Component)]
pub struct Simulated;

#[derive(Bundle)]
struct CelestialBodyBundle {
    body: CelestialBody,
    clicked: PickableBundle,
    raycast: RaycastPickable,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    click: On<Pointer<Click>>,
    hover: On<Pointer<Over>>,
    simulated: Simulated,
}

impl CelestialBodyBundle {
    fn new(
        body: CelestialBody,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let mesh =
            create_celestial_body_mesh(body.radius, body.get_surface_colour(), meshes, materials);
        Self {
            body,
            clicked: PickableBundle::default(),
            raycast: RaycastPickable,
            mesh,
            simulated: Simulated,
            click: On::<Pointer<Click>>::run(click_body),
            hover: On::<Pointer<Over>>::run(describe_body),
        }
    }
}

pub fn spawn_body(
    body: CelestialBody,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    info!("Spawning {:?}", &body);
    commands.spawn(CelestialBodyBundle::new(body, meshes, materials));
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

pub fn create_celestial_body_mesh(
    radius: f32,
    colour: Color,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> MaterialMesh2dBundle<ColorMaterial> {
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
