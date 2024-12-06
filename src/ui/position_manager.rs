use crate::ui::app_state::{AppState, FractalPosition};
use eframe::egui;

pub struct PositionManager;

impl PositionManager {
    pub fn show(state: &mut AppState, ctx: &egui::Context) {
        egui::SidePanel::right("positions_panel").show(ctx, |ui| {
            ui.separator();
            ui.heading("Positions sauvegardées");

            Self::save_button(state, ui);
            Self::save_dialog(state, ctx);
            Self::saved_positions_list(state, ui);
            Self::history_controls(state, ui);
        });
    }

    fn save_button(state: &mut AppState, ui: &mut egui::Ui) {
        if ui.button("Sauvegarder position").clicked() {
            state.show_save_dialog = true;
        }
    }

    fn save_dialog(state: &mut AppState, ctx: &egui::Context) {
        if state.show_save_dialog {
            egui::Window::new("Sauvegarder la position")
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Nom:");
                        ui.text_edit_singleline(&mut state.new_position_name);
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Sauvegarder").clicked() && !state.new_position_name.is_empty() {
                            let position = FractalPosition {
                                x_center: state.x_center,
                                y_center: state.y_center,
                                view_width: state.view_width,
                                view_height: state.view_height,
                                fractal_type: state.fractal_type.clone(),
                                c_real: state.c_real,
                                c_imag: state.c_imag,
                                max_iterations: state.max_iterations,
                                palette: state.palette.clone(),
                            };
                            state.saved_positions.push((state.new_position_name.clone(), position));
                            state.new_position_name.clear();
                            state.show_save_dialog = false;
                        }
                        if ui.button("Annuler").clicked() {
                            state.show_save_dialog = false;
                            state.new_position_name.clear();
                        }
                    });
                });
        }
    }

    fn saved_positions_list(state: &mut AppState, ui: &mut egui::Ui) {
        ui.group(|ui| {
            // Collecte des actions à effectuer
            let mut action = None;

            // Création d'une copie des positions pour l'itération
            let positions: Vec<_> = state.saved_positions
                .iter()
                .enumerate()
                .map(|(i, (name, position))| (i, name.clone(), position.clone()))
                .collect();

            // Affichage de la liste
            for (index, name, position) in positions {
                ui.horizontal(|ui| {
                    if ui.button(&name).clicked() {
                        action = Some(SavedPositionAction::Restore(position));
                    }
                    if ui.button("🗑").clicked() {
                        action = Some(SavedPositionAction::Remove(index));
                    }
                });
            }

            // Application des actions après l'itération
            match action {
                Some(SavedPositionAction::Restore(position)) => {
                    state.restore_position(&position);
                },
                Some(SavedPositionAction::Remove(index)) => {
                    state.saved_positions.remove(index);
                },
                None => {}
            }
        });
    }

    fn history_controls(state: &mut AppState, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Historique");
        if ui.button("Retour arrière").clicked() && !state.position_history.is_empty() {
            if let Some(previous) = state.position_history.pop_front() {
                state.restore_position(&previous);
            }
        }
    }
}

enum SavedPositionAction {
    Restore(FractalPosition),
    Remove(usize),
}