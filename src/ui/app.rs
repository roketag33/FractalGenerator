use crate::fractal::{animation, renderer::ProgressiveRenderer, generator};
use crate::audio::player;
use eframe::egui;
use super::{
    app_state::AppState,
    interaction::Interaction,
    position_manager::PositionManager,
    settings_panel::SettingsPanel
};

pub struct FractalApp {
    pub state: AppState,
    initialized: bool,
}

impl Default for FractalApp {
    fn default() -> Self {
        let mut app = Self {
            state: AppState::default(),
            initialized: false,
        };
        app.generate_fractal(); // Génération immédiate au démarrage
        app.initialized = true;
        app
    }
}

impl eframe::App for FractalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Génération initiale de la fractale
        if !self.initialized {
            println!("Initializing app..."); // Log
            self.generate_fractal();
            self.initialized = true;
            println!("App initialized"); // Log
        }

        // Affichage des paramètres
        SettingsPanel::show(self, ctx);

        // Gestion des positions
        PositionManager::show(&mut self.state, ctx);

        // Gestion des entrées souris et clavier
        Interaction::handle_mouse_input(self, ctx);
        animation::handle_keyboard_input(self, ctx);

        // Gestion de l'animation
        if self.state.is_animating {
            animation::animate_zoom(self);
            self.generate_fractal();
        }

        // Gestion du son
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            player::play_fractal_sound(self.state.zoom_level);
        }

        // Affichage de la fractale dans le panneau central
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            println!("Panel size: {:?}", size); // Log

            // Mise à jour des dimensions
            self.state.img_width = size.x as u32;
            self.state.img_height = size.y as u32;

            // Vérifier si une régénération est nécessaire
            let needs_regeneration = if let Some(image) = &self.state.image {
                image.size[0] != size.x as usize || image.size[1] != size.y as usize
            } else {
                true
            };

            // Régénérer si nécessaire
            if needs_regeneration {
                println!("Regeneration needed"); // Log
                self.state.renderer.reset();
                self.generate_fractal();
            }

            // Afficher l'image si elle existe
            if let Some(image) = &self.state.image {
                println!("Displaying image with size: {:?}", image.size); // Log
                let texture = ui.ctx().load_texture(
                    "fractale",
                    image.clone(),
                    egui::TextureOptions::NEAREST,
                );
                ui.image(&texture, size);
            } else {
                println!("No image to display"); // Log
            }
        });

        // Demander un rafraîchissement continu si nécessaire
        if self.state.renderer.should_render() {
            ctx.request_repaint();
        }
    }
}

impl FractalApp {
    pub fn generate_fractal(&mut self) {
        // Forcer le rendu initial
        if !self.initialized || self.state.renderer.should_render() {
            let resolution = self.state.renderer.next_resolution();

            println!("Generating fractal with resolution: {}", resolution); // Log

            let width = self.state.img_width / resolution;
            let height = self.state.img_height / resolution;

            if width == 0 || height == 0 {
                println!("Error: Invalid dimensions: {}x{}", width, height); // Log
                return;
            }

            println!("Dimensions: {}x{}", width, height); // Log

            let image = generator::generate_image(
                width,
                height,
                self.state.x_center - self.state.view_width / 2.0,
                self.state.x_center + self.state.view_width / 2.0,
                self.state.y_center - self.state.view_height / 2.0,
                self.state.y_center + self.state.view_height / 2.0,
                self.state.max_iterations,
                &self.state.palette,
                &self.state.fractal_type,
                self.state.c_real,
                self.state.c_imag,
            );

            println!("Image generated successfully"); // Log
            self.state.image = Some(image);
        } else {
            println!("Skipping render - not needed"); // Log
        }
    }
}