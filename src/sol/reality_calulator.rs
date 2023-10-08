
use super::CelestialBody;
use crate::sol::Vector2D;
use vector2d::Vector2D;

pub fn get_object_with_most_mass(objects: Vec<CelestialBody>) -> &CelestialBody {
    objects
        .iter()
        .reduce(|acc, body| if acc.mass >= body.mass { acc } else { body })
        .unwrap()
}

pub fn get_distance(body_a: &CelestialBody, body_b: &CelestialBody) -> f32 {
    (body_a.pos - body_b.pos).length()
}

pub fn calculate_new_positions(mut objects: Vec<CelestialBody>) {
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

    // TODO
    self.time = self.time + TIME_DELTA_PER_TICK as f64;
}
