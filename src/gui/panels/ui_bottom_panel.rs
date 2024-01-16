use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Vec2},
    EguiContexts,
};
use bevy_mod_picking::prelude::*;

use crate::{
    boundry::create_celestial_body_scene,
    gui::{
        assets::asset_loader::SceneAssets,
        kb_mouse::mouse_states::{LeftClickActionState, UIMouseState},
        tools::spawning::{EndSpawningEvent, SpawningBody, StartSpawningEvent, UIPlaceState},
    },
    sol::{celestial_body::CelestialBody, celestial_type::CelestialType},
};

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
    mut commands: Commands,
    mut picked_body: ResMut<UIPickedBody>,
    // query: Query<&CelestialBody>,
    spawning_entity: Query<Entity, With<SpawningBody>>,
    mut mouse_state: ResMut<UIMouseState>,
    mut start_spawning: EventWriter<StartSpawningEvent>,
    mut end_spawning: EventWriter<EndSpawningEvent>,
    mut place_state: ResMut<UIPlaceState>,
    mut egui_contexts: EguiContexts,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scene_assets: Res<SceneAssets>,
) {
    let asteroid_scene = create_celestial_body_scene(30.0, scene_assets);

    let control_spacer_width = 60.0;

    egui::Window::new(format!("Bottom Panel"))
        .resizable(false)
        .anchor(Align2::CENTER_BOTTOM, Vec2 { x: 0.0, y: -10.0 })
        .fixed_size(Vec2 { x: 100.0, y: 600.0 })
        // .default_width(400.0)
        .resizable(false)
        .title_bar(false)
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                // Asteroids
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // https://github.com/mvlabat/bevy_egui/blob/v0.20.1/examples/ui.rs
                        // ui.add(egui::widgets::Image::new(
                        //     *rendered_texture_id,
                        //     [256.0, 256.0],
                        // ));
                        // ui.menu_button("X", |ui| {
                        //     if ui.button("X").clicked() {
                        //     }
                        // });

                        if mouse_state.left == LeftClickActionState::Spawning {
                            if ui
                                .add_enabled(true, egui::Button::new("**Spawning**"))
                                .clicked()
                            {
                                // remove_spawning_body(&mut commands, &mut mouse_state, spawning_entity);
                                // Keep this, stops multiple spawning entities which causes a crash in the cursor bind fn
                                end_spawning.send(EndSpawningEvent);
                            }
                        } else {
                            if ui.add_enabled(true, egui::Button::new("Spawn")).clicked() {
                                // spawn_spawning_body(CelestialType::ASTEROID, &mut mouse_state, &mut commands);
                                start_spawning.send(StartSpawningEvent(CelestialType::ASTEROID));
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Asteroid"));
                    });
                });
                ui.add_space(control_spacer_width);
                // Planets
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // TODO Image of Planet
                        if mouse_state.left == LeftClickActionState::Spawning {
                            if ui
                                .add_enabled(true, egui::Button::new("**Spawning**"))
                                .clicked()
                            {
                                end_spawning.send(EndSpawningEvent);
                            }
                        } else {
                            if ui.add_enabled(true, egui::Button::new("Spawn")).clicked() {
                                start_spawning.send(StartSpawningEvent(CelestialType::PLANET));
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Planet"));
                    });
                });
                ui.add_space(control_spacer_width);
                // Stars
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // TODO Image of Star
                        if mouse_state.left == LeftClickActionState::Spawning {
                            if ui
                                .add_enabled(true, egui::Button::new("**Spawning**"))
                                .clicked()
                            {
                                end_spawning.send(EndSpawningEvent);
                            }
                        } else {
                            if ui.add_enabled(true, egui::Button::new("Spawn")).clicked() {
                                start_spawning.send(StartSpawningEvent(CelestialType::STAR));
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Star"));
                    });
                });
            });
        });
}
