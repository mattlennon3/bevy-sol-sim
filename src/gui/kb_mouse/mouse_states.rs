use bevy::prelude::*;

#[derive(Resource)]
pub struct UIMouseState {
  pub left: LeftClickActionState,
  pub right: RightClickActionState
}

impl UIMouseState {
  fn new() -> UIMouseState {
    UIMouseState {
      left: LeftClickActionState::Selecting,
      right: RightClickActionState::Selecting
    }
  }
}

impl Default for UIMouseState {
  fn default() -> Self {
      Self::new()
  }
}

// impl Plugin for UIMouseState {
//   fn build(&self, app: &mut App) {
//       app.insert_resource(UIMouseState::new());
//   }
// }


#[derive(Default, Eq, PartialEq)]
pub enum LeftClickActionState {
  #[default]
  Selecting,
  Spawning,
  // Dragging,??
  Removing,
  // Empty, ??
}

#[derive(Default, Eq, PartialEq)]
pub enum RightClickActionState {
  #[default]
  Selecting,
  // Removing,
  // Empty,
}

