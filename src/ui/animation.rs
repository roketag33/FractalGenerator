use crate::ui::app::FractalApp;
use eframe::egui;

/// Applique un zoom progressif sur la fractale.
pub fn animate_zoom(app: &mut FractalApp) {
    let zoom_factor = 1.05; // Facteur de zoom
    app.state.zoom_level *= zoom_factor;

    // Réduire la taille de la vue en fonction du facteur de zoom
    app.state.view_width /= zoom_factor;
    app.state.view_height /= zoom_factor;
}

/// Gère les déplacements de la vue avec les touches fléchées.
pub fn handle_keyboard_input(app: &mut FractalApp, ctx: &egui::Context) {
    // Déplacement proportionnel à la taille actuelle de la vue
    let step_x = app.state.view_width * 0.1; // 10% de la largeur actuelle
    let step_y = app.state.view_height * 0.1; // 10% de la hauteur actuelle

    // Flèche gauche : décaler le centre vers la gauche
    if ctx.input(|i| i.key_down(egui::Key::ArrowLeft)) {
        app.state.x_center -= step_x;
    }
    // Flèche droite : décaler le centre vers la droite
    if ctx.input(|i| i.key_down(egui::Key::ArrowRight)) {
        app.state.x_center += step_x;
    }
    // Flèche haut : décaler le centre vers le haut
    if ctx.input(|i| i.key_down(egui::Key::ArrowUp)) {
        app.state.y_center += step_y;
    }
    // Flèche bas : décaler le centre vers le bas
    if ctx.input(|i| i.key_down(egui::Key::ArrowDown)) {
        app.state.y_center -= step_y;
    }
}