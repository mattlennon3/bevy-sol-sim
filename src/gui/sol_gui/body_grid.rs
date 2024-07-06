// Plugin to render a vector grid for a Celestial Body

use bevy::prelude::*;

use crate::{
    gui::assets::mesher_plugin::{render_radius_layer, render_voxel_vertical_boundary},
    sol::{
        celestial_body::Voxelised,
        voxels::composition::{calculate_voxels_per_layer, Composition, VOXEL_SQUARE},
    },
};

#[derive(Component)]
pub struct ShowGrid;

// use super::asset_loader::SceneAssets;
pub struct BodyGridPlugin;

impl Plugin for BodyGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_body_grid);
    }
}

// TODO: Improve this by using the four corner function
fn add_body_grid(
    query: Query<(&Voxelised, &Composition, &Transform), With<ShowGrid>>,
    mut gizmos: Gizmos,
) {
    if query.is_empty() {
        return;
    }

    for (_, composition, transform) in query.iter() {
        for radius in composition.get_core_radius()..composition.get_max_radius() {
            render_radius_layer(radius, transform, &mut gizmos);
            let voxels_in_layer = calculate_voxels_per_layer(radius);
            render_voxel_vertical_boundary(radius, voxels_in_layer, transform, &mut gizmos);
        }
    }
}
