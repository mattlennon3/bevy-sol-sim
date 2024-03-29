use std::fmt::{Debug, Display, Error, Formatter};
use bevy::prelude::Component;

#[derive(PartialEq, Clone, Copy, Component)]
pub enum CelestialType {
    STAR,
    PLANET,
    // MOON,
    ASTEROID,
}

impl Display for CelestialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CelestialType::STAR => write!(f, "STAR"),
            CelestialType::PLANET => write!(f, "PLANET"),
            CelestialType::ASTEROID => write!(f, "ASTEROID"),
        }
    }
}

impl Debug for CelestialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CelestialType::STAR => write!(f, "STAR"),
            CelestialType::PLANET => write!(f, "PLANET"),
            CelestialType::ASTEROID => write!(f, "ASTEROID"),
        }
    }
}
