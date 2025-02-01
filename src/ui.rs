use crate::app::{FractalApp, ActiveFractal, ActiveColorScheme, ActiveColorSchemeType};
use crate::fractal_calculator::FractalCalculator;
use crate::fractal_types::{MandelbrotSet, JuliaSet, BurningShip, Tricorn, NewtonSet};
use eframe::egui;

pub struct UserInterface;

impl UserInterface {
    pub fn update(app: &mut FractalApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Sauvegarder l'image").clicked() {
                        app.save_dialog = Some(rfd::FileDialog::new()
                            .add_filter("PNG", &["png"])
                            .set_directory("~")
                            .set_file_name("fractal.png"));
                        ui.close_menu();
                    }
                });

                ui.menu_button("Type de fractale", |ui| {
                    if ui.radio(matches!(app.active_fractal, ActiveFractal::Mandelbrot(_)), "Mandelbrot").clicked() {
                        let params = app.params.clone();
                        app.set_fractal_type(ActiveFractal::Mandelbrot(
                            FractalCalculator::<MandelbrotSet>::new_mandelbrot(params)
                        ));
                    }
                    if ui.radio(matches!(app.active_fractal, ActiveFractal::Julia(_)), "Julia").clicked() {
                        let params = app.params.clone();
                        app.set_fractal_type(ActiveFractal::Julia(
                            FractalCalculator::<JuliaSet>::new_julia(params, app.params.julia_c)
                        ));
                    }
                    if ui.radio(matches!(app.active_fractal, ActiveFractal::BurningShip(_)), "Burning Ship").clicked() {
                        let params = app.params.clone();
                        app.set_fractal_type(ActiveFractal::BurningShip(
                            FractalCalculator::<BurningShip>::new_burning_ship(params)
                        ));
                    }
                    if ui.radio(matches!(app.active_fractal, ActiveFractal::Tricorn(_)), "Tricorn").clicked() {
                        let params = app.params.clone();
                        app.set_fractal_type(ActiveFractal::Tricorn(
                            FractalCalculator::<Tricorn>::new_tricorn(params)
                        ));
                    }
                    if ui.radio(matches!(app.active_fractal, ActiveFractal::Newton(_)), "Newton").clicked() {
                        let params = app.params.clone();
                        app.set_fractal_type(ActiveFractal::Newton(
                            FractalCalculator::<NewtonSet>::new_newton(params)
                        ));
                    }
                });

                ui.menu_button("Palette de couleurs", |ui| {
                    if ui.radio(matches!(app.active_color_scheme, ActiveColorScheme::Classic(_)), "Classic").clicked() {
                        app.set_color_scheme(ActiveColorSchemeType::Classic);
                    }
                    if ui.radio(matches!(app.active_color_scheme, ActiveColorScheme::Fire(_)), "Fire").clicked() {
                        app.set_color_scheme(ActiveColorSchemeType::Fire);
                    }
                    if ui.radio(matches!(app.active_color_scheme, ActiveColorScheme::Ocean(_)), "Ocean").clicked() {
                        app.set_color_scheme(ActiveColorSchemeType::Ocean);
                    }
                    if ui.radio(matches!(app.active_color_scheme, ActiveColorScheme::Rainbow(_)), "Rainbow").clicked() {
                        app.set_color_scheme(ActiveColorSchemeType::Rainbow);
                    }
                    if ui.radio(matches!(app.active_color_scheme, ActiveColorScheme::Grayscale(_)), "Grayscale").clicked() {
                        app.set_color_scheme(ActiveColorSchemeType::Grayscale);
                    }
                });
            });

            // Contrôles
            ui.horizontal(|ui| {
                ui.label("Zoom:");
                if ui.add(egui::Slider::new(&mut app.params.zoom, 0.1..=10.0)).changed() {
                    // Mettre à jour le calculateur avec les nouveaux paramètres
                    let params = app.params.clone();
                    app.active_fractal = match &app.active_fractal {
                        ActiveFractal::Mandelbrot(_) => {
                            ActiveFractal::Mandelbrot(FractalCalculator::<MandelbrotSet>::new_mandelbrot(params))
                        },
                        ActiveFractal::Julia(_) => {
                            ActiveFractal::Julia(FractalCalculator::<JuliaSet>::new_julia(params, app.params.julia_c))
                        },
                        ActiveFractal::BurningShip(_) => {
                            ActiveFractal::BurningShip(FractalCalculator::<BurningShip>::new_burning_ship(params))
                        },
                        ActiveFractal::Tricorn(_) => {
                            ActiveFractal::Tricorn(FractalCalculator::<Tricorn>::new_tricorn(params))
                        },
                        ActiveFractal::Newton(_) => {
                            ActiveFractal::Newton(FractalCalculator::<NewtonSet>::new_newton(params))
                        },
                    };
                    app.need_update = true;
                }
                
                ui.label("Iterations:");
                if ui.add(egui::Slider::new(&mut app.params.max_iterations, 10..=1000)).changed() {
                    app.need_update = true;
                }
            });

            // Gestion du zoom à la molette
            if ui.ui_contains_pointer() {
                ui.input(|i| {
                    let scroll_delta = i.scroll_delta.y;
                    if scroll_delta != 0.0 {
                        let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
                        if let Some(mouse_pos) = i.pointer.hover_pos() {
                            app.handle_zoom(mouse_pos.x, mouse_pos.y, zoom_factor);
                        }
                    }
                });
            }

            // Traitement de la sauvegarde
            if let Some(dialog) = app.save_dialog.take() {
                if let Some(path) = dialog.save_file() {
                    app.save_image(&path);
                }
            }

            // Affichage de la fractale
            let available_size = ui.available_size();
            let new_size = (
                available_size.x.round() as usize,
                available_size.y.round() as usize
            );

            // Mettre à jour la taille si nécessaire
            if app.params.size != new_size {
                app.params.size = new_size;
                app.need_update = true;
            }

            // Générer la fractale si nécessaire
            if app.need_update {
                app.generate_fractal();
                app.need_update = false;
            }

            // Créer et afficher l'image
            if !app.image_data.is_empty() {
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [app.params.size.0, app.params.size.1],
                    &app.image_data
                );
                let texture = ui.ctx().load_texture(
                    "fractal",
                    color_image,
                    Default::default()
                );
                ui.image(&texture);
            }
        });
    }
} 