use num::Complex;
use rayon::prelude::*;
use crate::fractal_params::FractalParams;
use crate::fractal_types::{FractalFunction, MandelbrotSet, JuliaSet, BurningShip, Tricorn, NewtonSet};
use crate::color_schemes::ColorScheme;

#[derive(Clone)]
pub struct FractalCalculator<F: FractalFunction> {
    params: FractalParams,
    fractal: F,
}

impl<F: FractalFunction + Sync> FractalCalculator<F> {
    pub fn new(params: FractalParams, fractal: F) -> Self {
        Self { params, fractal }
    }

    pub fn generate(&self, color_scheme: &(dyn ColorScheme + Sync)) -> Vec<u8> {
        let (width, height) = self.params.size;
        let mut image_data = vec![0u8; width * height * 4];

        let pixels: Vec<(usize, (u8, u8, u8))> = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width).into_par_iter().map(move |x| {
                    let cx = (x as f64 / width as f64 - 0.5) / self.params.zoom + self.params.center.re;
                    let cy = (y as f64 / height as f64 - 0.5) / self.params.zoom + self.params.center.im;
                    let c = Complex::new(cx, cy);
                    
                    let (iterations, z) = self.fractal.iterate(
                        c, 
                        self.fractal.initial_z(), 
                        self.params.max_iterations
                    );
                    
                    let color = color_scheme.smooth_color(iterations, self.params.max_iterations, z.norm_sqr());
                    (y * width + x, color)
                })
            })
            .collect();

        // Remplir image_data...
        pixels.into_iter().for_each(|(index, color)| {
            let pixel_index = index * 4;
            image_data[pixel_index..pixel_index + 3].copy_from_slice(&[color.0, color.1, color.2]);
            image_data[pixel_index + 3] = 255;
        });

        image_data
    }

    pub fn new_mandelbrot(params: FractalParams) -> FractalCalculator<MandelbrotSet> {
        FractalCalculator::<MandelbrotSet>::new(params, MandelbrotSet)
    }

    pub fn new_julia(params: FractalParams, c: Complex<f64>) -> FractalCalculator<JuliaSet> {
        FractalCalculator::<JuliaSet>::new(params, JuliaSet { c })
    }

    pub fn new_with_type(params: FractalParams, fractal_type: F) -> Self {
        Self {
            params,
            fractal: fractal_type,
        }
    }

    pub fn new_burning_ship(params: FractalParams) -> FractalCalculator<BurningShip> {
        FractalCalculator::<BurningShip>::new(params, BurningShip)
    }

    pub fn new_tricorn(params: FractalParams) -> FractalCalculator<Tricorn> {
        FractalCalculator::<Tricorn>::new(params, Tricorn)
    }

    pub fn new_newton(params: FractalParams) -> FractalCalculator<NewtonSet> {
        FractalCalculator::<NewtonSet>::new(params, NewtonSet)
    }
} 