use num::Complex;
use rayon::prelude::*;
use crate::fractal_params::FractalParams;
use crate::fractal_types::{FractalFunction, MandelbrotSet, JuliaSet, BurningShip, Tricorn, NewtonSet};
use crate::color_schemes::ColorScheme;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Clone)]
pub struct FractalCalculator<F: FractalFunction> {
    params: FractalParams,
    fractal: F,
    thread_count: usize,
    quality_level: QualityLevel,
    cache: HashMap<(i32, i32), (u32, Complex<f64>)>,
    cache_enabled: bool,
    shared_buffer: Option<SharedMemoryBuffer>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum QualityLevel {
    Low,    // 1/4 résolution
    Medium, // 1/2 résolution
    High,   // Pleine résolution
}

#[derive(Clone)]
pub struct AdaptiveRenderer {
    min_detail_level: u32,
    max_detail_level: u32,
    error_threshold: f64,
}

#[derive(Clone)]
pub struct PrecomputedData {
    escape_radius: Vec<f64>,
    color_lookup: Vec<(u8, u8, u8)>,
}

#[derive(Clone)]
pub struct SharedMemoryBuffer {
    data: Arc<RwLock<Vec<u8>>>,
    size: (usize, usize),
}

impl<F: FractalFunction + Sync> FractalCalculator<F> {
    pub fn new(params: FractalParams, fractal: F) -> Self {
        Self {
            params,
            fractal,
            thread_count: 4,
            quality_level: QualityLevel::High,
            cache: HashMap::new(),
            cache_enabled: true,
            shared_buffer: None,
        }
    }

    pub fn generate(&self, color_scheme: &(dyn ColorScheme + Sync)) -> Vec<u8> {
        let (width, height) = self.params.size;
        let mut image_data = vec![0u8; width * height * 4];

        let scale = match self.quality_level {
            QualityLevel::Low => 4,
            QualityLevel::Medium => 2,
            QualityLevel::High => 1,
        };

        let scaled_width = width / scale;
        let scaled_height = height / scale;

        let pixels: Vec<(usize, (u8, u8, u8))> = (0..scaled_height)
            .into_par_iter()
            .with_max_len(height / self.thread_count)
            .flat_map(|y| {
                (0..scaled_width).into_par_iter().map(move |x| {
                    let cx = (x as f64 / scaled_width as f64 - 0.5) / self.params.zoom + self.params.center.re;
                    let cy = (y as f64 / scaled_height as f64 - 0.5) / self.params.zoom + self.params.center.im;
                    let c = Complex::new(cx, cy);
                    
                    let key = (x as i32, y as i32);
                    let (iterations, z) = if let Some(&result) = self.cache.get(&key) {
                        result
                    } else {
                        self.fractal.iterate(c, self.fractal.initial_z(), self.params.max_iterations)
                    };
                    
                    let color = color_scheme.smooth_color(iterations, self.params.max_iterations, z.norm_sqr());
                    (y * scaled_width + x, color)
                })
            })
            .collect();

        for (index, color) in pixels {
            let base_x = (index % scaled_width) * scale;
            let base_y = (index / scaled_width) * scale;
            
            for dy in 0..scale {
                for dx in 0..scale {
                    let pixel_idx = ((base_y + dy) * width + base_x + dx) * 4;
                    if pixel_idx + 3 < image_data.len() {
                        image_data[pixel_idx..pixel_idx + 3].copy_from_slice(&[color.0, color.1, color.2]);
                        image_data[pixel_idx + 3] = 255;
                    }
                }
            }
        }

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
            thread_count: 4,
            quality_level: QualityLevel::High,
            cache: HashMap::new(),
            cache_enabled: true,
            shared_buffer: None,
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

    pub fn set_quality_level(&mut self, quality: QualityLevel) {
        self.quality_level = quality;
    }

    pub fn set_thread_count(&mut self, count: usize) {
        self.thread_count = count;
    }

    pub fn enable_cache(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
        if !enabled {
            self.cache.clear();
        }
    }

    fn optimize_complex_ops(&self, z: Complex<f64>) -> Complex<f64> {
        let re = z.re;
        let im = z.im;
        let re2 = re * re;
        let im2 = im * im;
        Complex::new(re2 - im2, 2.0 * re * im)
    }

    fn fast_norm(&self, z: Complex<f64>) -> f64 {
        z.re * z.re + z.im * z.im
    }

    pub fn with_shared_buffer(mut self) -> Self {
        let (width, height) = self.params.size;
        self.shared_buffer = Some(SharedMemoryBuffer {
            data: Arc::new(RwLock::new(vec![0; width * height * 4])),
            size: (width, height),
        });
        self
    }
} 