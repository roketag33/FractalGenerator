use crate::ui::app::FractalApp;
use eframe::egui;

pub struct SettingsPanel;

impl SettingsPanel {
    pub fn show(app: &mut FractalApp, ctx: &egui::Context) {
        egui::SidePanel::left("paramètres").show(ctx, |ui| {
            ui.heading("Paramètres de la fractale");

            // Paramètres de qualité
            Self::quality_settings(app, ui);

            // Paramètres de vue
            Self::view_settings(app, ui);

            // Type de fractale et paramètres spécifiques
            Self::fractal_type_settings(app, ui);

            // Choix de la palette
            Self::palette_settings(app, ui);

            // Contrôles d'animation
            Self::animation_controls(app, ui);
        });
    }

    fn quality_settings(app: &mut FractalApp, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Qualité");
            ui.checkbox(&mut app.state.super_sampling, "Super-sampling");
            if app.state.super_sampling {
                ui.add(egui::Slider::new(&mut app.state.super_sampling_factor, 2..=4)
                    .text("Facteur"));
            }
        });
    }

    fn view_settings(app: &mut FractalApp, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Position de la vue:");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut app.state.x_center).speed(0.1));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut app.state.y_center).speed(0.1));
            });

            ui.label("Taille de la vue:");
            ui.horizontal(|ui| {
                ui.label("Largeur:");
                ui.add(egui::DragValue::new(&mut app.state.view_width).speed(0.1));
                ui.label("Hauteur:");
                ui.add(egui::DragValue::new(&mut app.state.view_height).speed(0.1));
            });
        });
    }

    fn fractal_type_settings(app: &mut FractalApp, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Type:");
                egui::ComboBox::from_id_source("fractal_type_combo")
                    .selected_text(&app.state.fractal_type)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.fractal_type, "Mandelbrot".to_string(), "Mandelbrot");
                        ui.selectable_value(&mut app.state.fractal_type, "Julia".to_string(), "Julia");
                    });
            });

            if app.state.fractal_type == "Julia" {
                ui.label("Paramètres de Julia:");
                ui.horizontal(|ui| {
                    ui.label("c_real:");
                    ui.add(egui::DragValue::new(&mut app.state.c_real).speed(0.01));
                });
                ui.horizontal(|ui| {
                    ui.label("c_imag:");
                    ui.add(egui::DragValue::new(&mut app.state.c_imag).speed(0.01));
                });
            }

            ui.horizontal(|ui| {
                ui.label("Itérations:");
                ui.add(egui::Slider::new(&mut app.state.max_iterations, 10..=1000));
            });
        });
    }

    fn palette_settings(app: &mut FractalApp, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Palette:");
                egui::ComboBox::from_id_source("palette_combo")
                    .selected_text(&app.state.palette)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.palette, "Bleu-Rouge".to_string(), "Bleu-Rouge");
                        ui.selectable_value(&mut app.state.palette, "Arc-en-ciel".to_string(), "Arc-en-ciel");
                        ui.selectable_value(&mut app.state.palette, "Nuances de Gris".to_string(), "Nuances de Gris");
                    });
            });
        });
    }

    fn animation_controls(app: &mut FractalApp, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Générer").clicked() {
                    app.state.renderer.reset();
                    app.generate_fractal();
                    app.state.save_to_history();
                }
                if ui.button(if app.state.is_animating { "Arrêter" } else { "Animer" }).clicked() {
                    app.state.is_animating = !app.state.is_animating;
                }
            });
        });
    }
}