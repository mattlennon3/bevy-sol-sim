use std::alloc::System;

use vector2d::Vector2D;

use super::celestial_body::CelestialBody;

pub type SystemContents = Vec<CelestialBody>;

// Change to 6.67e-11 for real world
pub const GRAVITY: f32 = 0.5;
// pub const TIME_START: f32 = 0.0;
pub const TIME_DELTA_PER_TICK: f32 = 0.01;

// pub fn get_trajectory_path(body: &CelestialBody, system: SystemContents) -> Vec<Vector2D<f32>> {
//     // return body.trail.clone();
// }

pub fn get_object_with_most_mass(objects: &Vec<CelestialBody>) -> &CelestialBody {
    objects
        .iter()
        .reduce(|acc, body| if acc.mass >= body.mass { acc } else { body })
        .unwrap()
}

pub fn get_distance(body_a: &CelestialBody, body_b: &CelestialBody) -> f32 {
    (body_a.pos - body_b.pos).length()
}

pub fn calculate_new_positions(
    all_objects: &Vec<CelestialBody>,
    mut objects: Vec<CelestialBody>,
) -> Vec<CelestialBody> {
    for body in &mut objects {
        let mut forces: Vec<Vector2D<f32>> = vec![];

        for other_body in all_objects {
            // Check against all other bodies apart from the current one
            // **TODO** Check this actually works
            if other_body != body {
                forces.push(body.get_force_vector(&other_body));
            }
        }
        let cumulitive_forces: Vector2D<f32> = forces
            .iter()
            .fold(Vector2D { x: 0.0, y: 0.0 }, |acc, x| acc + *x);

        // TIME_DELTA_PER_TICK ==  Step value? (no, feels like it would be the wrong value and change the course of the object by updating less often)
        // also wtf is this calculation??? I feel like TIME_DELTA_PER_TICK should be 1
        body.momentum = body.momentum + cumulitive_forces * TIME_DELTA_PER_TICK;
        body.pos = body.pos + body.momentum / body.mass * TIME_DELTA_PER_TICK;

        // Trail
        body.trail.push(body.pos);

        if body.trail.len() > 100 {
            body.trail.remove(0);
        }
    }

    return objects;

    // TODO
    // self.time = self.time + TIME_DELTA_PER_TICK as f64;
}

pub fn one_planet_system() -> SystemContents {
    let star_mass: f32 = 2.0 * 1000.0;
    let star = CelestialBody::new_star(
        Vector2D { x: 0.0, y: 0.0 },
        Vector2D { x: 0.0, y: 0.0 },
        star_mass,
    );
    let planet = CelestialBody::new_planet(
        Vector2D { x: 850.0, y: 0.0 },
        Vector2D { x: 0.0, y: 1330.0 },
        30.0,
    );
    let objects = vec![star, planet];
    return objects;
}

pub fn default_system() -> SystemContents {
    let star_mass: f32 = 2.0 * 1000.0;
    let star = CelestialBody::new_star(
        Vector2D { x: 0.0, y: 0.0 },
        Vector2D { x: 0.0, y: 0.0 },
        star_mass,
    );
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
    let planet = CelestialBody::new_planet(
        Vector2D { x: 850.0, y: 0.0 },
        Vector2D { x: 0.0, y: 1330.0 },
        30.0,
    );
    let planet2 = CelestialBody::new_planet(
        Vector2D { x: 0.0, y: -300.0 },
        Vector2D { x: 1300.0, y: 0.0 },
        8.0,
    );
    let planet3 = CelestialBody::new_planet(
        Vector2D { x: 240.0, y: 0.0 },
        Vector2D { x: 0.0, y: 2000.0 },
        10.0,
    );
    let planet4 = CelestialBody::new_planet(
        Vector2D { x: 0.0, y: 450.0 },
        Vector2D { x: 1500.0, y: 0.0 },
        19.0,
    );
    let planet5 = CelestialBody::new_planet(
        Vector2D {
            x: 250.0,
            y: -500.0,
        },
        Vector2D { x: 700.0, y: 700.0 },
        20.0,
    );

    let objects = vec![star, planet, planet2, planet3, planet4, planet5];
    print_system(&objects, 0);
    return objects;
}

fn print_system(system: &SystemContents, time: u32) {
    println!("Solar System Time: {}", time);

    let name_width = 20;
    let body_type_width = 20;
    let mass_width = 20;
    let radius_width = 20;

    println!(
        "{:<name_width$} {:<body_type_width$} {:<mass_width$} {:<radius_width$}",
        "Name",
        "Body Type",
        "Mass",
        "Radius",
        name_width = name_width,
        body_type_width = body_type_width,
        mass_width = mass_width,
        radius_width = radius_width
    );
    println!(
        "{:_<name_width$} {:_<body_type_width$} {:_<mass_width$} {:_<radius_width$}",
        "",
        "",
        "",
        "",
        name_width = name_width,
        body_type_width = body_type_width,
        mass_width = mass_width,
        radius_width = radius_width
    );
    for body in system.iter() {
        println!(
            "{:<name_width$} {:<body_type_width$} {:<mass_width$} {:<radius_width$}",
            body.name,
            format!("{:?}", body.body_type),
            body.mass,
            body.radius,
            name_width = name_width,
            body_type_width = body_type_width,
            mass_width = mass_width,
            radius_width = radius_width
        );
    }
}
