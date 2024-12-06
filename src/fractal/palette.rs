use image::Rgb;

pub fn apply_palette(iteration: usize, max_iterations: usize, palette: &str) -> Rgb<u8> {
    match palette {
        "Arc-en-ciel" => {
            let hue = (iteration as f64 / max_iterations as f64 * 360.0) as u8;
            Rgb([hue, 255 - hue, hue / 2])
        }
        "Nuances de Gris" => {
            let intensity = (iteration as f64 / max_iterations as f64 * 255.0) as u8;
            Rgb([intensity, intensity, intensity])
        }
        _ => {
            // Palette par défaut : Bleu-Rouge
            let intensity = (iteration as f64 / max_iterations as f64 * 255.0) as u8;
            Rgb([intensity, 0, 255 - intensity])
        }
    }
}
