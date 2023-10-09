use std::fmt::{Debug, Display, Error, Formatter};

#[derive(PartialEq, Clone, Copy)]
pub enum CelestialType {
    STAR,
    PLANET,
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
