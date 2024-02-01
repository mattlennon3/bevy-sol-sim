use bevy::ecs::prelude::*;


#[derive(PartialEq, Clone, Copy, Component)]
pub enum ElementType {
    ROCK,
}


#[derive(PartialEq, Clone, Copy, Component)]
pub struct SolElement {
  element_type: ElementType,
}

impl SolElement {
  pub fn new(element_type: ElementType) -> Self {
    SolElement {
      element_type
    }
  }

  pub fn rock() -> Self {
    SolElement::new(ElementType::ROCK)
  }

  pub fn get_mass(&self) -> f32 {
    match self.element_type {
      ElementType::ROCK => ROCK_MASS,
    }
  }

  // get_melting_point
  // get_boiling_point
  // get_hardness
}

static ROCK_MASS: f32 = 100.0;
static ROCK_MELTING_POINT: u32 = u32::MAX;

static IRON_MASS: f32 = 300.0;
// static IRON_MELTING_POINT: u32 = 300;