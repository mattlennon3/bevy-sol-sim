use bevy::prelude::*;

#[derive(Resource)]
pub struct UIMouseState {
  left: LeftClickActionState,
  right: RightClickActionState
}

impl UIMouseState {
  fn new() -> UIMouseState {
    UIMouseState {
      left: LeftClickActionState::Selecting,
      right: RightClickActionState::Selecting
    }
  }
}

impl Plugin for UIMouseState {
  fn build(&self, app: &mut App) {
      app.insert_resource(UIMouseState::new());
  }
}


#[derive(Default)]
enum LeftClickActionState {
  #[default]
  Selecting,
  Spawning,
  // Dragging,??
  Removing,
  // Empty, ??
}

#[derive(Default)]
enum RightClickActionState {
  #[default]
  Selecting,
  // Removing,
  // Empty,
}

