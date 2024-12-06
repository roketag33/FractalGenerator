use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use crate::fractal::palette;

/// Génère une image de la fractale en fonction des paramètres donnés.
pub fn generate_image(
    img_width: u32,
    img_height: u32,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iterations: usize,
    palette: &str,
    fractal_type: &str,
    c_real: f64,
    c_imag: f64,
) -> egui::ColorImage {
    // Création d'une nouvelle image vide
    let mut img = ImageBuffer::new(img_width, img_height);

    // Calcul parallèle des pixels
    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            // Conversion des coordonnées pixel en coordonnées du plan complexe
            let cx = x_min + (x as f64 / img_width as f64) * (x_max - x_min);
            let cy = y_min + (y as f64 / img_height as f64) * (y_max - y_min);

            // Initialisation des variables selon le type de fractale
            let (mut zx, mut zy, real_c, imag_c) = match fractal_type {
                // Pour Julia, on utilise les coordonnées comme point de départ
                // et les paramètres c_real/c_imag comme constante
                "Julia" => (cx, cy, c_real, c_imag),
                // Pour Mandelbrot, on commence à (0,0) et on utilise les coordonnées
                // comme constante
                _ => (0.0, 0.0, cx, cy),
            };

            let mut iteration = 0;

            // Calcul des itérations pour le point courant
            while zx * zx + zy * zy < 4.0 && iteration < max_iterations {
                let temp = zx * zx - zy * zy + real_c;
                zy = 2.0 * zx * zy + imag_c;
                zx = temp;
                iteration += 1;
            }

            // Application de la couleur selon la palette choisie
            *pixel = palette::apply_palette(iteration, max_iterations, palette);
        });

    // Conversion en ColorImage pour egui
    egui::ColorImage::from_rgb(
        [img_width as usize, img_height as usize],
        &img.into_raw(),
    )
}