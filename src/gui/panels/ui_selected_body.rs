use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Vec2},
    EguiContexts,
};
use bevy_mod_picking::prelude::*;

use crate::sol::{
    celestial_body::{celestial_body::speed, Mass, Momentum, BodyName, Radius},
    celestial_type::CelestialType,
};

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
    query: Query<&CelestialType>,
    event: Listener<Pointer<Over>>,
) {
    if let Ok(_body) = query.get(event.target) {
        active_body.selected = Some(event.target.clone());
    }
}

pub fn render_active_body_gui(
    active_body: Res<UISelectedBody>,
    query: Query<(&CelestialType, &Transform, &Mass, &BodyName, &Radius, &Momentum)>,
    mut egui_contexts: EguiContexts,
) {
    match active_body.selected {
        Some(selected) => {
            if let Ok((body_type, transform, mass, name, radius, momentum)) = query.get(selected) {
                egui::Window::new(format!("Info: {:?}", name))
                    // .scroll2([false, true])
                    // .vscroll(true)
                    .fixed_size(Vec2 { x: 100.0, y: 100.0 })
                    // .default_width(400.0)
                    .resizable(false)
                    .anchor(Align2::RIGHT_TOP, Vec2 { x: -10.0, y: 50.0 })
                    .show(egui_contexts.ctx_mut(), |ui| {
                        let pos = transform.translation;
                        ui.horizontal(|ui| {
                            ui.label(format!("Type: {:?}", body_type));
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!("Name: {:?}", name));
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!("Mass: {:?}", mass));
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!("Radius: {:.4}", radius.0));
                        });
                        ui.horizontal(|ui| {
                            // TODO: What is this unit?
                            ui.label(format!("Speed: {:.4} mps", speed(momentum, mass)));
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!("Pos: x:{:.4}, y:{:.4}", pos.x, pos.y));
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!("Momentum: x:{:.4}, y:{:.4}", momentum.0.x, momentum.0.y));
                        });
                    });
            }
        }
        None => (),
    };
}
