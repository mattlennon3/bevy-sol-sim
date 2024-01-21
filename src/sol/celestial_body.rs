use bevy::prelude::*;
use rand::Rng;
use rnglib::{Language, RNG};
use std::ops::Sub;

use crate::{
    gui::{constants::constants::Z_LEVELS, sol_gui::celestial_body_gui::CelestialBodyGuiBundle},
    sol::reality_calculator::Simulated,
};

use super::{celestial_type::CelestialType, reality_calculator::GRAVITY};

#[derive(Component, Clone, Eq, PartialEq, Debug)]
pub struct Name(String);
#[derive(Component, Clone, PartialEq)]
pub struct Radius(pub f32);

#[derive(Component, Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Mass(pub f32);

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Distance(pub f32);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position(pub Vec2);

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct Momentum(pub Vec2);

#[derive(Bundle, Clone, PartialEq)]
pub struct CelestialBodyBundle {
    pub body_type: CelestialType,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub name: Name,
    pub radius: Radius,
    pub mass: Mass,
    // Not sure if momentum should be in the initial bundle
    // pub momentum: Momentum,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Vec2 { x, y })
    }
    pub fn new_from_transform(transform: &Transform) -> Self {
        Position(Vec2 {
            x: transform.translation.x,
            y: transform.translation.y,
        })
    }
}

impl Momentum {
    pub fn new(x: f32, y: f32) -> Self {
        Momentum(Vec2 { x, y })
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let Position(lhs) = self;
        let Position(rhs) = rhs;
        Position::new(lhs.x - rhs.x, lhs.y - rhs.y)
    }
}

pub trait TransformUpdates2d {
    fn new_body(pos: Position) -> Self;
    fn update_position(&mut self, pos: Position);
    fn update_position_vec2(&mut self, pos: Vec2);
}

impl TransformUpdates2d for Transform {
    fn new_body(pos: Position) -> Self {
        let pos: Vec2 = pos.into();
        Transform {
            translation: Vec3::new(pos.x, pos.y, Z_LEVELS.foreground),
            ..Default::default()
        }
    }
    fn update_position(&mut self, pos: Position) {
        let pos: Vec2 = pos.into();
        self.translation = Vec3::new(pos.x, pos.y, Z_LEVELS.foreground);
    }
    fn update_position_vec2(&mut self, pos: Vec2) {
        self.translation = Vec3::new(pos.x, pos.y, Z_LEVELS.foreground);
    }
}

impl From<Transform> for Position {
    fn from(transform: Transform) -> Self {
        let Transform { translation, .. } = transform;
        Position::new(translation.x, translation.y)
    }
}

// impl From<Position> for Transform {
//     fn from_(position: Position, z_level: Option<f32>) -> Self {
//         Transform {  
//             translation: Vec3 { x: position.0.x, y: position.0.y}
//             ..Default::default()
//         }
//     }
// }

impl From<Position> for Vec2 {
    fn from(pos: Position) -> Self {
        let Position(vec_position) = pos;
        vec_position
    }
}

impl CelestialBodyBundle {
    pub fn new(body_type: CelestialType, position: Position, mass: Option<Mass>) -> Self {
        let name = Name(celestial_body::get_default_name_for_body(body_type));
        let mass = match mass {
            Some(mass) => mass,
            None => celestial_body::random_mass(body_type),
        };
        let radius = celestial_body::get_default_radius(body_type, mass);
        let transform = Transform::new_body(position);
        Self {
            body_type,
            transform,
            global_transform: GlobalTransform::from(transform),
            name,
            mass,
            radius,
        }
    }

    pub fn new_asteroid(position: Position, mass: Option<Mass>) -> Self {
        CelestialBodyBundle::new(CelestialType::ASTEROID, position, mass)
    }

    pub fn new_planet(position: Position, mass: Option<Mass>) -> Self {
        CelestialBodyBundle::new(CelestialType::PLANET, position, mass)
    }

    pub fn new_star(position: Position, mass: Option<Mass>) -> Self {
        CelestialBodyBundle::new(CelestialType::STAR, position, mass)
    }
}

// convert fully into bundle, and put all of this out into components???
// #[derive(PartialEq, Clone, Component)]
// pub struct CelestialBody {
//     // pub name: String,
//     // pub mass: f32,
//     // pub body_type: CelestialType,
//     // pub radius: f32,
//     // pub trail: VecDeque<Vector2D<f32>>,
//     // pub momentum: Vec2,
// }

// TODO Make CelestialBody a private struct, for the helper methods?
// NO, will have to always make new struct for each system call

pub mod celestial_body {
    use super::*;
    use crate::sol::celestial_type::CelestialType;

    pub fn random_mass(body_type: CelestialType) -> Mass {
        let floor = match body_type {
            CelestialType::ASTEROID => 1,
            CelestialType::PLANET => 50,
            CelestialType::STAR => 700,
        };
        let ceiling = match body_type {
            CelestialType::ASTEROID => 5,
            CelestialType::PLANET => 600,
            CelestialType::STAR => 3500,
        };
        // generate number between floor and ceiling
        Mass(rand::thread_rng().gen_range(floor..ceiling) as f32)
    }

    // pub fn new_random(body_type: CelestialType, momentum: Vec2) -> Self {
    //     let floor = match body_type {
    //         CelestialType::ASTEROID => 1,
    //         CelestialType::PLANET => 50,
    //         CelestialType::STAR => 700,
    //     };
    //     let ceiling = match body_type {
    //         CelestialType::ASTEROID => 5,
    //         CelestialType::PLANET => 600,
    //         CelestialType::STAR => 3500,
    //     };
    //     // generate number between floor and ceiling
    //     let mass = rand::thread_rng().gen_range(floor..ceiling) as f32;
    //     CelestialBody::new(body_type, mass, momentum)
    // }

    #[derive(Debug, Clone, Copy)]
    pub struct ForceVectorData {
        pub pos: Position,
        pub mass: Mass,
    }

    pub fn get_force_vector(first: &ForceVectorData, second: &ForceVectorData) -> Vec2 {
        // Thanks very much to Lets Code Physics
        // https://www.youtube.com/watch?v=4ycpvtIio-o&list=PLdCdV2GBGyXOExPW4u8H88S5mwrx_8vWK&index=3
        // I had to guess the force_vector code but amazingly it worked first time
        let distance_vec = first.pos - second.pos;
        let magnitude = distance_vec.0.length();
        
        let unit_vector = distance_vec.0 / magnitude;
        
        let force_magnitude = (GRAVITY * first.mass.0 * second.mass.0 / magnitude).powf(2.0);
        
        // KEEP THIS DEBUG BLOCK - helpful for when two things have the same position and it makes the magnitude infinite
        if magnitude.eq(&0.0) || force_magnitude.is_infinite() {
            dbg!(first, second, magnitude, force_magnitude);
            panic!("Force vector is infinite!");
        }
        // ---------------------

        let force_vector: Vec2 = Vec2 {
            x: -force_magnitude * unit_vector.x,
            y: -force_magnitude * unit_vector.y,
        };

        return force_vector;
    }

    pub fn get_default_radius(body_type: CelestialType, mass: Mass) -> Radius {
        // TODO: Do something with mass

        const SMALL_MASS_PLANET: Mass = Mass(15.0);
        // // small mass planets
        // R = M^0.55
        // // large mass bodies
        // R = M^0.01

        // let pow: f32 = match mass < SMALL_MASS_PLANET {
        //     true => 0.55,
        //     false => 0.01,
        // };

        // return mass.powf(pow);

        match body_type {
            CelestialType::STAR => Radius(22.0),
            CelestialType::PLANET => Radius(8.0),
            CelestialType::ASTEROID => Radius(1.),
        }
    }

    pub fn get_default_name_for_body(body_type: CelestialType) -> String {
        let star_name_gen: RNG = RNG::try_from(&Language::Roman).unwrap();
        let planet_name_gen: RNG = RNG::try_from(&Language::Fantasy).unwrap();
        let misc_name_gen: RNG = RNG::try_from(&Language::Curse).unwrap();
        match body_type {
            CelestialType::STAR => star_name_gen.generate_name().to_owned(),
            CelestialType::PLANET => planet_name_gen.generate_name().to_owned(),
            _ => misc_name_gen.generate_name().to_owned(),
        }
    }

    pub fn speed(momentum: &Momentum, mass: &Mass) -> f32 {
        momentum.0.length() / mass.0
    }

    // pub fn get_radius(&self) -> f32 {
    //     self.radius
    // }

    // pub fn get_gravity(&self) -> f32 {
    //     !todo!("Calculate the gravity based on mass");
    // }

    // TODO: Make material colour depend on body type and mass
    // maybe actually move to the celestialtype, but put in the constructor and set on the struct?
    pub fn get_surface_colour(body_type: &CelestialType) -> Color {
        match body_type {
            CelestialType::STAR => Color::YELLOW,
            CelestialType::PLANET => Color::SEA_GREEN,
            _ => Color::ORANGE,
        }
    }
}

// impl Debug for CelestialBody {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         write!(
//             f,
//             "CelestialBody {{ name: {}, body_type: {:?}, mass: {}, radius: {} }}",
//             self.name, self.body_type, self.mass, self.radius
//         )
//     }
// }

// impl Display for CelestialBody {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         write!(
//             f,
//             "CelestialBody {{ name: {}, body_type: {:?}, mass: {}, radius: {} }}",
//             self.name, self.body_type, self.mass, self.radius
//         )
//     }
// }
