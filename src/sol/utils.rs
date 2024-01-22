use super::celestial_body::{Mass, Position, Distance};

// Use system now and get MostMass component
// pub fn get_object_with_most_mass(objects: &Vec<Mass>) -> &Mass {
//   objects
//       .iter()
//       .reduce(|acc, mass: &Mass| if acc.0 >= mass.0 { acc } else { mass })
//       .unwrap()
// }

pub fn get_distance(body_a: &Position, body_b: &Position) -> Distance {
  Distance((body_a.0 - body_b.0).length())
}
