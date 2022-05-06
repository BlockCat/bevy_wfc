use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use rand::SeedableRng;
use wfc_solver::Solution;

use crate::wfc_asset::WfcProblemResource;

pub struct UIPlugin;

struct UIState {
    error_message: Option<String>,
    random_seed: bool,
    seed: u64,
}

#[derive(Component)]
pub struct LoadedSolution(pub Solution<String>);

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EguiPlugin)
            .add_system(gui_system)
            .insert_resource(UIState {
                error_message: None,
                random_seed: true,
                seed: 0,
            });
    }
}

fn gui_system(
    time: Res<Time>,
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UIState>,
    wfc: Option<ResMut<WfcProblemResource>>,
) {
    if let Some(mut wfc) = wfc {
        egui::Window::new("Generation").show(egui_context.ctx_mut(), |ui| {
            if let Some(error_message) = &ui_state.error_message {
                ui.label(error_message);
            }

            egui::Grid::new("Generation").show(ui, |ui| {
                ui.end_row();
                ui.label("Width");
                ui.label("Height");
                ui.label("Depth");
                ui.end_row();
                ui.add(egui::DragValue::new(&mut wfc.description.dimensions.width));
                ui.add(egui::DragValue::new(&mut wfc.description.dimensions.height));
                ui.add(egui::DragValue::new(&mut wfc.description.dimensions.depth));
                ui.end_row();
            });

            ui.label("Random");

            ui.add(egui::Checkbox::new(
                &mut ui_state.random_seed,
                "Use random seed",
            ));

            ui.add(egui::DragValue::new(&mut ui_state.seed));

            if ui.button("Generate").clicked() {
                let mut rng = if ui_state.random_seed {
                    rand::rngs::SmallRng::seed_from_u64(
                        ((time.seconds_since_startup() * 1000.0) % (std::u64::MAX as f64)) as u64,
                    )
                } else {
                    rand::rngs::SmallRng::seed_from_u64(ui_state.seed)
                };
                match wfc_solver::solve(&mut rng, wfc.description.clone()) {
                    Ok(solution) => {
                        commands.spawn().insert(LoadedSolution(solution));
                        ui_state.error_message = None;
                    }
                    Err(err) => ui_state.error_message = Some(format!("Failed solving! {}", err)),
                }
            }
        });
    }
}
