pub trait ColorScheme: Send + Sync {
    fn get_color(&self, t: f64) -> (u8, u8, u8);
    fn smooth_color(&self, iterations: u32, max_iterations: u32, z_norm: f64) -> (u8, u8, u8) {
        if iterations == max_iterations {
            return (0, 0, 0);
        }
        
        let log_zn = z_norm.ln() / 2.0;
        let nu = (log_zn / f64::ln(2.0)).ln() / f64::ln(2.0);
        let smooth_iter = iterations as f64 + 1.0 - nu;
        
        let t = smooth_iter / max_iterations as f64;
        self.get_color(t)
    }
}

pub struct ClassicScheme;
pub struct FireScheme;
pub struct OceanScheme;
pub struct RainbowScheme;
pub struct GrayscaleScheme;
// ... autres schémas de couleurs ... 

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;
    
    let (r, g, b) = match (h * 6.0).floor() as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

impl ColorScheme for ClassicScheme {
    fn get_color(&self, t: f64) -> (u8, u8, u8) {
        hsv_to_rgb(t as f32, 0.8, 1.0)
    }
}

impl ColorScheme for FireScheme {
    fn get_color(&self, t: f64) -> (u8, u8, u8) {
        let t = t as f32;
        let r = (t * 255.0) as u8;
        let g = ((t * t) * 255.0) as u8;
        let b = ((t * t * t) * 255.0) as u8;
        (r, g, b)
    }
}

impl ColorScheme for OceanScheme {
    fn get_color(&self, t: f64) -> (u8, u8, u8) {
        let t = t as f32;
        let b = (t * 255.0) as u8;
        let g = ((t * t) * 255.0) as u8;
        let r = ((t * t * t) * 255.0) as u8;
        (r, g, b)
    }
}

impl ColorScheme for RainbowScheme {
    fn get_color(&self, t: f64) -> (u8, u8, u8) {
        let hue = (t * 6.0) as f32;
        hsv_to_rgb(hue.fract(), 1.0, 1.0)
    }
}

impl ColorScheme for GrayscaleScheme {
    fn get_color(&self, t: f64) -> (u8, u8, u8) {
        let v = (t * 255.0) as u8;
        (v, v, v)
    }
} 