use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{sol::celestial_body::CelestialBody};

use super::asset_loader::SceneAssets;

pub struct MesherPlugin;

impl Plugin for MesherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, celestial_body_mesher);
    }
}

/** Add texture to each celestial body */
fn celestial_body_mesher(
    b_query: Query<(Entity, &CelestialBody), Without<Mesh2dHandle>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if b_query.is_empty() {
        return;
    }
    for (entity, body) in b_query.iter() {
        let mesh = create_celestial_body_mesh(
            body.radius,
            body.get_surface_colour(),
            &mut meshes,
            &mut materials,
        );
        commands.entity(entity).insert(mesh);
    }
}

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