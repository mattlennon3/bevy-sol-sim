use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::sol::{
    celestial_body::{celestial_body::get_surface_colour, Radius, Voxelised},
    celestial_type::CelestialType, voxels::composition::{calculate_voxels_per_layer, VOXEL_SQUARE},
};

pub struct MesherPlugin;

impl Plugin for MesherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid_gizmo_config);
        app.add_systems(Update, celestial_body_mesher);
    }
}

/** Add texture to each celestial body */
fn celestial_body_mesher(
    b_query: Query<
        (Entity, &CelestialType, &Radius, &Transform),
        (Without<Mesh2dHandle>, Without<Voxelised>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if b_query.is_empty() {
        return;
    }
    for (entity, body_type, radius, transform) in b_query.iter() {
        let mesh = create_celestial_body_mesh(
            radius,
            get_surface_colour(body_type),
            &transform,
            &mut meshes,
            &mut materials,
        );
        commands.entity(entity).insert(mesh);
    }
}

pub fn create_celestial_body_mesh(
    radius: &Radius,
    colour: Color,
    transform: &Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
        material: materials.add(ColorMaterial::from(colour)),
        transform: *transform,
        ..default()
    }
}

pub fn setup_grid_gizmo_config(
    mut config: ResMut<GizmoConfig>
) {
    config.line_width = 0.1;
}




// TODO: Improve this by using the four corner function
pub fn render_radius_layer(
    radius: u32,
    transform: &Transform,
    gizmos: &mut Gizmos,
) {
    let translation = transform.translation;
    let position = Vec2::new(translation.x, translation.y);
    // The *10 here is a total hack, because it looked horrendous without it
    let segments = calculate_voxels_per_layer(radius) * 10;
    gizmos.circle_2d(position, (radius * VOXEL_SQUARE) as f32, Color::ANTIQUE_WHITE).segments(segments as usize);
}

// TODO: Improve this by using the four corner function
pub fn render_voxel_vertical_boundary(
    radius: u32,
    voxel_quantity: u32,
    transform: &Transform,
    gizmos: &mut Gizmos,
) {
    let inner_radius = radius * VOXEL_SQUARE;
    let next_radius = (radius + 1) * VOXEL_SQUARE;
    
    // at equal intervals around the circle (voxel_quantity), draw a line from inner_radius to next_radius
    let translation = transform.translation;
    let center = Vec2::new(translation.x, translation.y);

    dbg!(radius, voxel_quantity);
    
    let angle_step = 2.0 * std::f32::consts::PI / voxel_quantity as f32;
    dbg!(angle_step);
    
    for voxel in 0..voxel_quantity {
        let angle = angle_step * voxel as f32;
        let start = center + Vec2::new(angle.cos(), angle.sin()) * inner_radius as f32;
        let end = center + Vec2::new(angle.cos(), angle.sin()) * next_radius as f32;
        let colour = match voxel == 0 {
            true => Color::RED,
            false => Color::ANTIQUE_WHITE,
        };
        gizmos.line_2d(start, end, colour);
    }
    
}

pub fn fill_voxel(
    radius: u32,
    offset: u32,
    transform: &Transform,
    gizmos: &mut Gizmos,
) {
    let inner_radius = radius * VOXEL_SQUARE;
    let next_radius = (radius + 1) * VOXEL_SQUARE;
    
    
    
}
