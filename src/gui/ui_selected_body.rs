use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_egui::{egui::{self, Align2, Vec2}, EguiContexts};

use crate::sol::celestial_body::CelestialBody;

#[derive(Resource)]
pub struct UISelectedBody {
    pub selected: Option<Entity>,
}

impl Default for UISelectedBody {
    fn default() -> Self {
        Self { selected: None }
    }
}

pub fn describe_body(
    mut active_body: ResMut<UISelectedBody>,
    query: Query<&CelestialBody>,
    event: Listener<Pointer<Over>>,
) {
    if let Ok(_body) = query.get(event.target) {
        active_body.selected = Some(event.target.clone());
    }
}

pub fn render_active_body_gui(
  active_body: Res<UISelectedBody>,
  query: Query<&CelestialBody>,
  mut egui_contexts: EguiContexts,
) {
  match active_body.selected {
      Some(selected) => {
          if let Ok(body) = query.get(selected) {
              egui::Window::new(format!("Info: {:?}", body.name))
              // .scroll2([false, true])
              // .vscroll(true)
              .fixed_size(Vec2 { x: 100.0, y: 100.0 })
              // .default_width(400.0)
              .resizable(false)
              .anchor(Align2::RIGHT_TOP, Vec2 { x: -10.0, y: 50.0 })
              .show(egui_contexts.ctx_mut(), |ui| {
                  ui.horizontal(|ui| {
                      ui.label(format!("Type: {:?}", body.body_type));
                  });
                  ui.horizontal(|ui| {
                      ui.label(format!("Name: {:?}", body.name));
                  });
                  ui.horizontal(|ui| {
                      ui.label(format!("Mass: {:?}", body.mass));
                  });
                  ui.horizontal(|ui| {
                      // TODO: What is this unit?
                      ui.label(format!("Speed: {:.4} mps", body.speed()));
                  });
                  ui.horizontal(|ui| {
                      ui.label(format!("Pos: x:{:.4}, y:{:.4}", body.pos.x, body.pos.y));
                  });
              });
          }
      }
      None => (),
  };
}
