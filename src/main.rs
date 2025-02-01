mod app;
mod fractal_params;
mod fractal_types;
mod color_schemes;
mod fractal_calculator;
mod ui;

use app::FractalApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Générateur de Fractales"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Générateur de Fractales",
        options,
        Box::new(|_cc| Box::new(FractalApp::default())),
    )
} 