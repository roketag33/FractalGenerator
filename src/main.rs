mod fractal;
mod audio;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::vec2(800.0, 600.0)); // Taille initiale explicite

    eframe::run_native(
        "Générateur de fractales interactif",
        options,
        Box::new(|_cc| Box::new(ui::app::FractalApp::default())),
    )
}