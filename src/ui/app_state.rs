use crate::fractal::{cache::FractalCache, renderer::ProgressiveRenderer};
use eframe::egui;
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Clone, Serialize, Deserialize)]
pub struct FractalPosition {
    pub x_center: f64,
    pub y_center: f64,
    pub view_width: f64,
    pub view_height: f64,
    pub fractal_type: String,
    pub c_real: f64,
    pub c_imag: f64,
    pub max_iterations: usize,
    pub palette: String,
}

pub struct AppState {
    // Paramètres de vue
    pub x_center: f64,
    pub y_center: f64,
    pub view_width: f64,
    pub view_height: f64,
    pub zoom_level: f64,

    // Paramètres de la fractale
    pub fractal_type: String,
    pub max_iterations: usize,
    pub c_real: f64,
    pub c_imag: f64,
    pub palette: String,

    // Paramètres d'affichage
    pub img_width: u32,
    pub img_height: u32,
    pub image: Option<egui::ColorImage>,
    pub is_animating: bool,

    // Gestion de l'historique et des positions sauvegardées
    pub position_history: VecDeque<FractalPosition>,
    pub saved_positions: Vec<(String, FractalPosition)>,
    pub history_max_size: usize,

    // Interface utilisateur
    pub drag_start_pos: Option<egui::Pos2>,
    pub show_save_dialog: bool,
    pub new_position_name: String,

    // Performance features
    pub cache: FractalCache,
    pub renderer: ProgressiveRenderer,
    pub super_sampling: bool,
    pub super_sampling_factor: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            x_center: -0.5,
            y_center: 0.0,
            view_width: 3.0,
            view_height: 2.0,
            zoom_level: 1.0,

            fractal_type: "Mandelbrot".to_string(),
            max_iterations: 100,
            c_real: -0.7,
            c_imag: 0.27015,
            palette: "Bleu-Rouge".to_string(),

            img_width: 800,
            img_height: 600,
            image: None,
            is_animating: false,

            position_history: VecDeque::with_capacity(50),
            saved_positions: Vec::new(),
            history_max_size: 50,

            drag_start_pos: None,
            show_save_dialog: false,
            new_position_name: String::new(),

            cache: FractalCache::new(100, 300),
            renderer: ProgressiveRenderer::new(1), // Changer la résolution cible à 1
            super_sampling: false,
            super_sampling_factor: 2,
        }
    }
}

impl AppState {
    pub fn save_to_history(&mut self) {
        let position = FractalPosition {
            x_center: self.x_center,
            y_center: self.y_center,
            view_width: self.view_width,
            view_height: self.view_height,
            fractal_type: self.fractal_type.clone(),
            c_real: self.c_real,
            c_imag: self.c_imag,
            max_iterations: self.max_iterations,
            palette: self.palette.clone(),
        };

        if self.position_history.len() >= self.history_max_size {
            self.position_history.pop_back();
        }
        self.position_history.push_front(position);
    }

    pub fn restore_position(&mut self, position: &FractalPosition) {
        self.x_center = position.x_center;
        self.y_center = position.y_center;
        self.view_width = position.view_width;
        self.view_height = position.view_height;
        self.fractal_type = position.fractal_type.clone();
        self.c_real = position.c_real;
        self.c_imag = position.c_imag;
        self.max_iterations = position.max_iterations;
        self.palette = position.palette.clone();
        self.renderer.reset();
    }
}