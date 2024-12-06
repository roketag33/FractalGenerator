use crate::ui::{
    app::FractalApp,
    app_state::AppState,
};use eframe::egui;

pub struct Interaction {
    pub zoom_factor_in: f64,
    pub zoom_factor_out: f64,
    pub movement_factor: f64,
}

impl Default for Interaction {
    fn default() -> Self {
        Self {
            zoom_factor_in: 0.9,
            zoom_factor_out: 1.1,
            movement_factor: 0.1,
        }
    }
}

impl Interaction {
    pub fn handle_mouse_input(app: &mut FractalApp, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.allocate_rect(
                ui.available_rect_before_wrap(),
                egui::Sense::click_and_drag(),
            );

            // Gestion du zoom avec la molette
            let scroll_delta = ctx.input(|i| i.scroll_delta.y);
            if scroll_delta != 0.0 {
                if let Some(pointer_pos) = response.hover_pos() {
                    let zoom_factor = if scroll_delta > 0.0 {
                        Self::default().zoom_factor_in
                    } else {
                        Self::default().zoom_factor_out
                    };

                    Self::handle_zoom(app, pointer_pos, response.rect, zoom_factor);
                }
            }

            // Gestion du drag and drop
            Self::handle_drag(app, &response);
        });
    }

    fn handle_zoom(
        app: &mut FractalApp,
        pointer_pos: egui::Pos2,
        rect: egui::Rect,
        zoom_factor: f64,
    ) {
        let x_ratio = (pointer_pos.x - rect.min.x) / rect.width();
        let y_ratio = (pointer_pos.y - rect.min.y) / rect.height();

        let x_target = app.state.x_center + (x_ratio as f64 - 0.5) * app.state.view_width;
        let y_target = app.state.y_center + (y_ratio as f64 - 0.5) * app.state.view_height;

        app.state.view_width *= zoom_factor;
        app.state.view_height *= zoom_factor;

        app.state.x_center = x_target + (app.state.x_center - x_target) * zoom_factor;
        app.state.y_center = y_target + (app.state.y_center - y_target) * zoom_factor;

        app.state.renderer.reset();
        app.state.save_to_history();
        app.generate_fractal();
    }

    fn handle_drag(app: &mut FractalApp, response: &egui::Response) {
        if response.drag_started() {
            app.state.drag_start_pos = response.hover_pos();
        }

        if response.dragged() {
            if let Some(start_pos) = app.state.drag_start_pos {
                let current_pos = response.hover_pos().unwrap_or(start_pos);
                let delta = current_pos - start_pos;

                let dx = delta.x as f64 * app.state.view_width / response.rect.width() as f64;
                let dy = delta.y as f64 * app.state.view_height / response.rect.height() as f64;

                app.state.x_center -= dx;
                app.state.y_center -= dy;
                app.state.renderer.reset();
                app.generate_fractal();

                app.state.drag_start_pos = Some(current_pos);
            }
        }

        if response.drag_released() {
            app.state.drag_start_pos = None;
            app.state.save_to_history();
        }
    }
}