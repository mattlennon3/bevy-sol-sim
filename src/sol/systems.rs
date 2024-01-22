use bevy::math::Vec2;

use super::celestial_body::{CelestialBodyBundle, Mass, Momentum, Position};

pub type SystemObject = (CelestialBodyBundle, Momentum);

pub type SystemContents = Vec<SystemObject>;

pub fn sol_system() -> SystemContents {
    let star_mass: Mass = Mass(2.0 * 1000.0);
    let star = CelestialBodyBundle::new_star(Position::new(0.0, 0.0), Some(star_mass));
    let star_momentum = Momentum(Vec2::ZERO);

    let objects = vec![(star, star_momentum)];
    return objects;
}

pub fn one_planet_system() -> SystemContents {
    let star_mass: Mass = Mass(2.0 * 1000.0);

    let star = CelestialBodyBundle::new_star(Position::new(0.0, 0.0), Some(star_mass));
    // let star_bundle = CelestialBodyBundle::new(star, Position::new(0.0, 0.0 ));
    let star_momentum = Momentum(Vec2::ZERO);

    let planet = CelestialBodyBundle::new_planet(Position::new(450.0, 0.0), Some(Mass(30.0)));
    // let planet_bundle = CelestialBodyBundle::new(planet, Position::new(850.0, 0.0));
    let planet_momentum = Momentum(Vec2::new(0.0, 7530.0));

    let objects = vec![(star, star_momentum), (planet, planet_momentum)];
    return objects;
}

pub fn twin_planet_system() -> SystemContents {
    let star_mass: Mass = Mass(2.0 * 1000.0);

    let star = CelestialBodyBundle::new_star(Position::new(0.0, 0.0), Some(star_mass));
    // let star_bundle = CelestialBodyBundle::new(star, Position::new(0.0, 0.0 ));
    let star_momentum = Momentum(Vec2::ZERO);

    let planet = CelestialBodyBundle::new_planet(Position::new(450.0, 0.0), Some(Mass(30.0)));
    // let planet_bundle = CelestialBodyBundle::new(planet, Position::new(850.0, 0.0));
    let planet_momentum = Momentum(Vec2::new(0.0, 7530.0));

    let planet2 = CelestialBodyBundle::new_planet(Position::new(-450.0, 0.0), Some(Mass(30.0)));
    // let planet_bundle = CelestialBodyBundle::new(planet, Position::new(850.0, 0.0));
    let planet2_momentum = Momentum(Vec2::new(0.0, -7530.0));

    let objects = vec![(star, star_momentum), (planet, planet_momentum), (planet2, planet2_momentum)];
    return objects;
}

// pub fn default_system() -> SystemContents {
//   let star_mass: f32 = 2.0 * 1000.0;
//   let star = CelestialBodyBundle::new_star(Position::new(0.0,  0.0), star_mass);
//   let star_bundle = CelestialBodyBundle::new(star, Position::new(0.0,  0.0));
//   // let star2 = CelestialBody {
//   //     name: CelestialBody::get_name(),
//   //     body_type: CelestialType::STAR,
//   //     mass: 2.0 * 800.0,
//   //     momentum: Vec2 {
//   //         x: 30000.0,
//   //         y: 0750000.0,
//   //     },
//   //     pos: ,
//   //     radius: 22.0,
//   //     trail: vec![],
//   // };
//   let planet = CelestialBody::new_planet(Position::new(0.0,  1330.0), 30.0);
//   let planet_bundle = CelestialBodyBundle::new(planet, Position::new(850.0,  0.0));

//   let planet2 = CelestialBody::new_planet(Position::new(1300.0,  0.0), 8.0);
//   let planet2_bundle = CelestialBodyBundle::new(planet2, Position::new(0.0,  -300.0));

//   let planet3 = CelestialBody::new_planet(Position::new(0.0,  2000.0), 10.0);
//   let planet3_bundle = CelestialBodyBundle::new(planet3, Position::new(240.0,  0.0));

//   let planet4 = CelestialBody::new_planet(Position::new(1500.0,  0.0), 19.0);
//   let planet4_bundle = CelestialBodyBundle::new(planet4, Position::new(0.0,  450.0));

//   let planet5 = CelestialBody::new_planet(Position::new(700.0,  700.0), 20.0);
//   let planet5_bundle = CelestialBodyBundle::new(
//       planet5,
//       Vec2 {
//           x: 250.0,
//           y: -500.0,
//       },
//   );

//   let objects = vec![star_bundle, planet_bundle, planet2_bundle, planet3_bundle, planet4_bundle, planet5_bundle];
//   print_system(&objects, 0);
//   return objects;
// }

// fn print_system(system: &SystemContents, time: u32) {
//   println!("Solar System Time: {}", time);

//   let name_width = 20;
//   let body_type_width = 20;
//   let mass_width = 20;
//   let radius_width = 20;

//   println!(
//       "{:<name_width$} {:<body_type_width$} {:<mass_width$} {:<radius_width$}",
//       "Name",
//       "Body Type",
//       "Mass",
//       "Radius",
//       name_width = name_width,
//       body_type_width = body_type_width,
//       mass_width = mass_width,
//       radius_width = radius_width
//   );
//   println!(
//       "{:_<name_width$} {:_<body_type_width$} {:_<mass_width$} {:_<radius_width$}",
//       "",
//       "",
//       "",
//       "",
//       name_width = name_width,
//       body_type_width = body_type_width,
//       mass_width = mass_width,
//       radius_width = radius_width
//   );
//   for body_bundle in system.iter() {
//       let body = body_bundle.body;
//       println!(
//           "{:<name_width$} {:<body_type_width$} {:<mass_width$} {:<radius_width$}",
//           body.name,
//           format!("{:?}", body.body_type),
//           body.mass,
//           body.radius,
//           name_width = name_width,
//           body_type_width = body_type_width,
//           mass_width = mass_width,
//           radius_width = radius_width
//       );
//   }
// }
