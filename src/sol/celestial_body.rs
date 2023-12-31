use std::fmt::{Debug, Display, Error, Formatter};

use rand::Rng;
use ::vector2d::Vector2D;
use bevy::prelude::{Component, Color, Event, Bundle};
use rnglib::{Language, RNG};

use super::{celestial_type::CelestialType, reality_calulator::GRAVITY};

#[derive(PartialEq, Clone, Component)]
pub struct CelestialBody {
    pub name: String,
    pub mass: f32,
    pub pos: Vector2D<f32>,
    pub body_type: CelestialType,
    pub radius: f32,
    pub trail: Vec<Vector2D<f32>>,
    pub momentum: Vector2D<f32>,
}

impl CelestialBody {
    pub fn new(
        body_type: CelestialType,
        pos: Vector2D<f32>,
        mass: f32,
        momentum: Vector2D<f32>,
    ) -> Self {
        let name = CelestialBody::get_default_name_for_body(body_type);
        let radius = CelestialBody::get_default_radius(body_type, mass);
        let trail = vec![];

        Self {
            name,
            body_type,
            mass,
            momentum,
            pos,
            radius,
            trail,
        }
    }

    pub fn new_random(body_type: CelestialType, pos: Vector2D<f32>, momentum: Vector2D<f32>) -> Self {
        let floor = match body_type {
            CelestialType::ASTEROID => 1,
            CelestialType::PLANET => 50,
            CelestialType::STAR => 700
        };
        let ceiling = match body_type {
            CelestialType::ASTEROID => 5,
            CelestialType::PLANET => 600,
            CelestialType::STAR => 3500
        };
        // generate number between floor and ceiling
        let mass = rand::thread_rng().gen_range(floor..ceiling) as f32;
        CelestialBody::new(body_type, pos, mass, momentum)
    }

    pub fn new_planet(pos: Vector2D<f32>, momentum: Vector2D<f32>, mass: f32) -> Self {
        CelestialBody::new(CelestialType::PLANET, pos, mass, momentum)
    }

    pub fn new_star(pos: Vector2D<f32>, momentum: Vector2D<f32>, mass: f32) -> Self {
        CelestialBody::new(CelestialType::STAR, pos, mass, momentum)
    }

    pub fn get_force_vector(&self, other: &CelestialBody) -> Vector2D<f32> {
        // Thanks very much to Lets Code Physics
        // https://www.youtube.com/watch?v=4ycpvtIio-o&list=PLdCdV2GBGyXOExPW4u8H88S5mwrx_8vWK&index=3
        // I had to guess the force_vector code but amazingly it worked first time
        let distance_vec = self.pos - other.pos;
        let magnitude = distance_vec.length();

        let unit_vector = distance_vec / magnitude;

        let force_magnitude = (GRAVITY * self.mass * other.mass / magnitude).powf(2.0);
        let force_vector: Vector2D<f32> = Vector2D {
            x: -force_magnitude * unit_vector.x,
            y: -force_magnitude * unit_vector.y,
        };
        return force_vector;
    }

    pub fn get_default_radius(body_type: CelestialType, mass: f32) -> f32 {
        // TODO: Do something with mass

        const SMALL_MASS_PLANET: f32 = 15.0;
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
            CelestialType::STAR => 22.0,
            CelestialType::PLANET => 8.0,
            CelestialType::ASTEROID => 0.01,
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

    pub fn speed(&self) -> f32 {
        self.momentum.length() / self.mass
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    // TODO: Make material colour depend on body type and mass
    // maybe actually move to the celestialtype, but put in the constructor and set on the struct?
    pub fn get_surface_colour(&self) -> Color {
        match self.body_type {
            CelestialType::STAR => Color::YELLOW,
            CelestialType::PLANET => Color::SEA_GREEN,
            _ => Color::GRAY,
        }
    }
}

impl Debug for CelestialBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "CelestialBody {{ name: {}, body_type: {:?}, mass: {}, radius: {} }}",
            self.name, self.body_type, self.mass, self.radius
        )
    }
}

impl Display for CelestialBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "CelestialBody {{ name: {}, body_type: {:?}, mass: {}, radius: {} }}",
            self.name, self.body_type, self.mass, self.radius
        )
    }
}
