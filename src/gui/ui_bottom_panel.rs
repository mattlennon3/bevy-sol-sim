use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Vec2},
    EguiContexts,
};
use bevy_mod_picking::prelude::*;

use crate::sol::celestial_body::CelestialBody;

pub trait InSimuation {
    fn run(&self);
}

#[derive(Resource)]
pub struct UIPickedBody {
    pub picked: Option<Entity>,
}

impl Default for UIPickedBody {
    fn default() -> Self {
        Self { picked: None }
    }
}

pub fn pick_body(
    mut picked_body: ResMut<UIPickedBody>,
    query: Query<&CelestialBody>,
    event: Listener<Pointer<Over>>,
) {
    if let Ok(_body) = query.get(event.target) {
        picked_body.picked = Some(event.target.clone());
    }
}

pub fn render_bottom_panel_gui(
    mut picked_body: ResMut<UIPickedBody>,
    // query: Query<&CelestialBody>,
    mut egui_contexts: EguiContexts,
) {
    egui::Window::new(format!("Bottom Panel"))
        .resizable(false)
        .anchor(Align2::CENTER_BOTTOM, Vec2 { x: 0.0, y: -10.0 })
        .fixed_size(Vec2 { x: 100.0, y: 600.0 })
        // .default_width(400.0)
        .resizable(false)
        .title_bar(false)
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("HI"));
            });
            // ui.horizontal(|ui| {
            //     ui.label(format!("Name: {:?}", body.name));
            // });
            // ui.horizontal(|ui| {
            //     ui.label(format!("Mass: {:?}", body.mass));
            // });
            // ui.horizontal(|ui| {
            //     // TODO: What is this unit?
            //     ui.label(format!("Speed: {:.4} mps", body.speed()));
            // });
            // ui.horizontal(|ui| {
            //     ui.label(format!("Pos: x:{:.4}, y:{:.4}", body.pos.x, body.pos.y));
            // });
        });
}
