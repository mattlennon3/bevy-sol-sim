use std::f32::consts::PI;

use bevy::{
    ecs::component::Component,
    log::warn,
    math::Vec2,
    transform::{self, components::Transform},
    utils::HashMap,
};

use crate::sol::celestial_body::Position;

use super::elements::SolElement;

// #[derive(PartialEq, Clone, Hash, Eq)]
// pub struct Angle(f32);

#[derive(PartialEq, Clone, Hash, Eq)]
pub struct VoxelPosition {
    radius: u32,
    // Offset can be the "angle"
    // clockwise from the top
    offset: u32,
}

#[derive(Clone, PartialEq, Component)]
pub struct Composition {
    // No fixed size
    // radius: u32,
    // core_radius: u32,
    // "fixed"
    core_mass: u32,
    voxels: HashMap<VoxelPosition, SolElement>,
}

impl Composition {
    pub fn new() -> Self {
        Composition {
            // radius: DEFAULT_CORE_RADIUS,
            core_mass: DEFAULT_CORE_MASS,
            voxels: HashMap::default(),
        }
    }

    pub fn rock_fill(&mut self, mass: f32) {
        let remaining_mass = mass - self.core_mass as f32;
        if remaining_mass <= 0.0 {
            warn!("Mass cannot be negative");
            dbg!(mass, self.core_mass, remaining_mass);
            return;
        }
        // add rocks until mass is reached
        let rock = SolElement::rock();
        let mut amount: u32 = (remaining_mass / rock.get_mass().round()) as u32;

        // Start from the outer-core
        let mut current_radius = self.get_core_radius() + 1;

        dbg!(amount);
        while amount > 0 {
            let voxels_in_layer = calculate_voxels_per_layer(current_radius);
            dbg!(current_radius, voxels_in_layer);

            // Add voxels to layer
            for offset in 0..voxels_in_layer {
                let voxel_position = VoxelPosition {
                    radius: current_radius,
                    offset,
                };
                self.add_voxel(voxel_position, rock.clone());
                if amount > 0 {
                    amount -= 1;
                }
            }
            current_radius += 1;
        }
    }

    fn add_voxel(&mut self, voxel_position: VoxelPosition, element: SolElement) {
        self.voxels.insert(voxel_position, element);
    }

    // convert from polar to cartesian
    pub fn get_voxel_cartesian_position(&self, voxel_position: VoxelPosition) -> Vec2 {
        let radius = voxel_position.radius;
        let offset = voxel_position.offset as f32;
        let angle = (offset / (calculate_voxels_per_layer(radius) as f32)) * 2.0 * PI;
        let radius = voxel_position.radius as f32;

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        Vec2::new(x, y)
    }

    pub fn get_core_radius(&self) -> u32 {
        DEFAULT_CORE_RADIUS
    }

    pub fn get_max_radius(&self) -> u32 {
        // get highest radius from voxels
        let peak = self
            .voxels
            .iter()
            .map(|(voxel_position, _)| voxel_position.radius)
            .max();
        if let Some(peak) = peak {
            peak
        } else {
            self.get_core_radius()
        }
    }

    /**
     *
     * TL         TR
     * |   ---    |
     * |          |
     * |   ---    |
     * BL         BR
     * 
     * // TODO: Offset alternating layers (avoid same starting line)
     * // This will create the brick layer effect
     */
    pub fn get_voxel_corners(&self, voxel: VoxelPosition, transform: Transform) -> Vec<Vec2> {
        let mut corners = Vec::new();
        let translation = transform.translation;
        let center = Vec2::new(translation.x, translation.y);

        let radius = voxel.radius;
        let offset = voxel.offset;

        let inner_radius = radius * VOXEL_SQUARE;
        let next_radius = (radius + 1) * VOXEL_SQUARE;

        let voxels_in_layer = calculate_voxels_per_layer(radius);
        let angle_step = 2.0 * std::f32::consts::PI / voxels_in_layer as f32;
        
        // TL BL
        let angle = angle_step * offset as f32;

            // TL
        let tl = center + Vec2::new(angle.cos(), angle.sin()) * inner_radius as f32;
        corners.push(tl);

             // BL
        let bl = center + Vec2::new(angle.cos(), angle.sin()) * inner_radius as f32;
        corners.push(bl);
        
        // TL BL
        // if offset == voxels_in_layer - 1 {
        //     offset = 0;
        // }
            
        let angle = angle_step * (offset + 1) as f32;
            // BR
        let br = center + Vec2::new(angle.cos(), angle.sin()) * next_radius as f32;
        corners.push(br);
            // TR
        let tr = center + Vec2::new(angle.cos(), angle.sin()) * next_radius as f32;
        corners.push(tr);
        
        return corners;
    }
}

pub fn calculate_voxels_per_layer(radius: u32) -> u32 {
    let circumference = 2.0 * PI * (radius as f32);
    let voxels_per_layer = circumference / (VOXEL_SQUARE as f32).round();
    dbg!(circumference);
    voxels_per_layer as u32 * VOXEL_SQUARE
}

// Both width and height in pixels
pub static VOXEL_SQUARE: u32 = 50;

static DEFAULT_CORE_MASS: u32 = 10000;
static DEFAULT_CORE_RADIUS: u32 = 50;
