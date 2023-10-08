use std::{
    fmt::{Debug, Error, Formatter, Display},
    ops::Add,
};

use ::vector2d::Vector2D;
use bevy::prelude::{Component, Name};
use rnglib::{Language, RNG};
// use regex::Regex;
// use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum CelestialType {
    STAR,
    PLANET,
}

#[derive(PartialEq, Clone, Component)]
pub struct CelestialBody {
    pub name: String,
    pub mass: f32,
    pub pos: Vector2D<f32>,
    pub body_type: CelestialType,
    pub radius: f32,
    pub trail: Vec<Vector2D<f32>>,
    momentum: Vector2D<f32>,
}

// Change to 6.67e-11 for real world
const GRAVITY: f32 = 0.5;
const TIME_START: f32 = 0.0;
const TIME_DELTA_PER_TICK: f32 = 0.01;

#[derive(Component)]
pub struct SolarSystem {
    // pub objects: Vec<CelestialBody>,
    pub time: f64,
}

pub fn default_system() -> Vec<CelestialBody> {
    let star = CelestialBody::new_star(Vector2D { x: 0.0, y: 0.0 }, Vector2D { x: 0.0, y: 0.0 });
    // let star2 = CelestialBody {
    //     name: CelestialBody::get_name(),
    //     body_type: CelestialType::STAR,
    //     mass: 2.0 * 800.0,
    //     momentum: Vector2D {
    //         x: 30000.0,
    //         y: 0750000.0,
    //     },
    //     pos: ,
    //     radius: 22.0,
    //     trail: vec![],
    // };
    let planet =
        CelestialBody::new_planet(Vector2D { x: 85.0, y: 0.0 }, Vector2D { x: 0.0, y: 130.0 });
    let planet2 = CelestialBody::new_planet(
        Vector2D { x: 0.0, y: -300.0 },
        Vector2D { x: 1300.0, y: 0.0 },
    );
    let planet3 = CelestialBody::new_planet(
        Vector2D { x: 240.0, y: 0.0 },
        Vector2D { x: 0.0, y: 2000.0 },
    );
    let planet4 = CelestialBody::new_planet(
        Vector2D { x: 0.0, y: 450.0 },
        Vector2D { x: 1500.0, y: 0.0 },
    );
    let planet5 = CelestialBody::new_planet(
        Vector2D {
            x: 250.0,
            y: -500.0,
        },
        Vector2D { x: 700.0, y: 700.0 },
    );

    let objects = vec![star, planet, planet2, planet3, planet4, planet5];
    return objects;
}

// TODO: Convert to a set of util functions, remove time

impl SolarSystem {
    pub fn new(objects: Option<Vec<CelestialBody>>) -> Self {
        Self {
            // objects: objects.unwrap_or(vec![]),
            time: TIME_START.into(),
        }
    }

    // pub fn add_object(&mut self, body: CelestialBody) {
    //     self.objects.push(body);
    // }

    /**
     * Get the center of mass of the solar system
     */
    // pub fn get_system_barycenter(&self) -> Vector2D<f32> {
    //     // total mass
    //     // get each objects % of total
    //     //
    //     let total_mass: f32 = self.objects.iter().map(|x| x.mass).sum();
    //     let mut total_momentum: Vector2D<f32> = Vector2D { x: 0.0, y: 0.0 };
    //     let barycenter = self.objects.iter().fold(total_momentum, |acc, x| {
    //         let momentum = x.momentum * x.mass;
    //         let total_momentum = acc + momentum;
    //         total_momentum
    //     }) / total_mass;
    //     // self.objects.iter()
    // }

    pub fn get_object_with_most_mass(&self, objects: Vec<CelestialBody>) -> &CelestialBody {
        objects
            .iter()
            .reduce(|acc, body| if acc.mass >= body.mass { acc } else { body })
            .unwrap()
    }

    pub fn get_distance(&self, body_a: &CelestialBody, body_b: &CelestialBody) -> f32 {
        (body_a.pos - body_b.pos).length()
    }

    // pub fn tick(&mut self) {
    //     self.calculate_new_positions();
    // }

    fn calculate_new_positions(&mut self, mut objects: Vec<CelestialBody>) {
        let all_objects = objects.clone();

        for body in &mut objects {
            let mut forces: Vec<Vector2D<f32>> = vec![];

            for other_body in &all_objects {
                // Check against all other bodies apart from the current one
                // **TODO** Check this actually works
                if other_body != body {
                    forces.push(body.get_force_vector(&other_body));
                }
            }
            let cumulitive_forces: Vector2D<f32> = forces
                .iter()
                .fold(Vector2D { x: 0.0, y: 0.0 }, |acc, x| acc + *x);

            body.momentum = body.momentum + cumulitive_forces * TIME_DELTA_PER_TICK;
            body.pos = body.pos + body.momentum / body.mass * TIME_DELTA_PER_TICK;

            // Trail
            body.trail.push(body.pos);

            if body.trail.len() > 100 {
                body.trail.remove(0);
            }
        }

        self.time = self.time + TIME_DELTA_PER_TICK as f64;
    }
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

    pub fn new_planet(pos: Vector2D<f32>, momentum: Vector2D<f32>) -> Self {
        let mass = 1.0;
        CelestialBody::new(CelestialType::PLANET, pos, mass, momentum)
    }

    pub fn new_star(pos: Vector2D<f32>, momentum: Vector2D<f32>) -> Self {
        let mass: f32 = 2.0 * 1000.0;
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
        match body_type {
            CelestialType::STAR => 22.0,
            CelestialType::PLANET => 8.0,
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

impl Display for CelestialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CelestialType::STAR => write!(f, "STAR"),
            CelestialType::PLANET => write!(f, "PLANET"),
        }
    }
}

impl Debug for CelestialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CelestialType::STAR => write!(f, "STAR"),
            CelestialType::PLANET => write!(f, "PLANET"),
        }
    }
}

impl Debug for SolarSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "\nSolar System Time: {}", self.time)?;

        let name_width = 20;
        let body_type_width = 20;
        let mass_width = 20;
        let radius_width = 20;

        writeln!(f, "{:<name_width$} {:<body_type_width$} {:<mass_width$} {:<radius_width$}", "Name", "Body Type", "Mass", "Radius", name_width=name_width, body_type_width=body_type_width, mass_width=mass_width, radius_width=radius_width)?;
        writeln!(f, "{:_<name_width$} {:_<body_type_width$} {:_<mass_width$} {:_<radius_width$}", "", "", "", "", name_width=name_width, body_type_width=body_type_width, mass_width=mass_width, radius_width=radius_width)?;
        for body in &self.objects {
            writeln!(f, "{:<name_width$} {:<body_type_width$} {:<mass_width$} {:<radius_width$}", body.name, format!("{:?}", body.body_type), body.mass, body.radius, name_width=name_width, body_type_width=body_type_width, mass_width=mass_width, radius_width=radius_width)?;
        }

        Ok(())
    }
}
