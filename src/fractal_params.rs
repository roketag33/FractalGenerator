use num::Complex;

#[derive(Clone)]
pub struct FractalParams {
    pub zoom: f64,
    pub center: Complex<f64>,
    pub max_iterations: u32,
    pub size: (usize, usize),
    pub julia_c: Complex<f64>,
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            center: Complex::new(-0.5, 0.0),
            max_iterations: 100,
            size: (800, 600),
            julia_c: Complex::new(-0.4, 0.6),
        }
    }
}