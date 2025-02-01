use crate::fractal_params::FractalParams;
use crate::fractal_types::{MandelbrotSet, JuliaSet, BurningShip, Tricorn, NewtonSet};
use crate::color_schemes::{ColorScheme, ClassicScheme, FireScheme, OceanScheme, RainbowScheme, GrayscaleScheme};
use crate::fractal_calculator::FractalCalculator;
use crate::ui::UserInterface;
use eframe::egui;
use num::Complex;
use image::{ImageBuffer, Rgb};
use std::path::PathBuf;

pub enum ActiveFractal {
    Mandelbrot(FractalCalculator<MandelbrotSet>),
    Julia(FractalCalculator<JuliaSet>),
    BurningShip(FractalCalculator<BurningShip>),
    Tricorn(FractalCalculator<Tricorn>),
    Newton(FractalCalculator<NewtonSet>),
}

pub enum ActiveColorScheme {
    Classic(ClassicScheme),
    Fire(FireScheme),
    Ocean(OceanScheme),
    Rainbow(RainbowScheme),
    Grayscale(GrayscaleScheme),
}

pub enum ActiveColorSchemeType {
    Classic,
    Fire,
    Ocean,
    Rainbow,
    Grayscale,
}

pub struct FractalApp {
    pub params: FractalParams,
    pub active_fractal: ActiveFractal,
    pub active_color_scheme: ActiveColorScheme,
    pub image_data: Vec<u8>,
    pub need_update: bool,
    pub save_dialog: Option<rfd::FileDialog>,
}

impl Default for FractalApp {
    fn default() -> Self {
        let params = FractalParams::default();
        Self {
            active_fractal: ActiveFractal::Mandelbrot(
                FractalCalculator::<MandelbrotSet>::new_mandelbrot(params.clone())
            ),
            active_color_scheme: ActiveColorScheme::Classic(ClassicScheme),
            params,
            image_data: Vec::new(),
            need_update: true,
            save_dialog: None,
        }
    }
}

impl eframe::App for FractalApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        UserInterface::update(self, ctx, frame);
    }
}

impl FractalApp {
    pub fn generate_fractal(&mut self) {
        let required_size = self.params.size.0 * self.params.size.1 * 4;
        if self.image_data.len() != required_size {
            self.image_data = vec![0; required_size];
        }

        let color_scheme: &(dyn ColorScheme + Sync) = match &self.active_color_scheme {
            ActiveColorScheme::Classic(scheme) => scheme,
            ActiveColorScheme::Fire(scheme) => scheme,
            ActiveColorScheme::Ocean(scheme) => scheme,
            ActiveColorScheme::Rainbow(scheme) => scheme,
            ActiveColorScheme::Grayscale(scheme) => scheme,
        };

        let new_data = match &self.active_fractal {
            ActiveFractal::Mandelbrot(calc) => calc.generate(color_scheme),
            ActiveFractal::Julia(calc) => calc.generate(color_scheme),
            ActiveFractal::BurningShip(calc) => calc.generate(color_scheme),
            ActiveFractal::Tricorn(calc) => calc.generate(color_scheme),
            ActiveFractal::Newton(calc) => calc.generate(color_scheme),
        };

        if new_data.len() == required_size {
            self.image_data = new_data;
        }
    }

    pub fn save_image(&self, path: &PathBuf) {
        let (width, height) = self.params.size;
        let mut img = ImageBuffer::new(width as u32, height as u32);
        
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let idx = (y as usize * width + x as usize) * 4;
            *pixel = Rgb([
                self.image_data[idx],
                self.image_data[idx + 1],
                self.image_data[idx + 2]
            ]);
        }
        
        img.save(path).expect("Échec de la sauvegarde de l'image");
    }

    pub fn set_fractal_type(&mut self, fractal_type: ActiveFractal) {
        self.active_fractal = fractal_type;
        self.need_update = true;
    }

    pub fn set_color_scheme(&mut self, scheme_type: ActiveColorSchemeType) {
        self.active_color_scheme = match scheme_type {
            ActiveColorSchemeType::Classic => ActiveColorScheme::Classic(ClassicScheme),
            ActiveColorSchemeType::Fire => ActiveColorScheme::Fire(FireScheme),
            ActiveColorSchemeType::Ocean => ActiveColorScheme::Ocean(OceanScheme),
            ActiveColorSchemeType::Rainbow => ActiveColorScheme::Rainbow(RainbowScheme),
            ActiveColorSchemeType::Grayscale => ActiveColorScheme::Grayscale(GrayscaleScheme),
        };
        self.need_update = true;
    }

    pub fn handle_zoom(&mut self, mouse_x: f32, mouse_y: f32, zoom_factor: f64) {
        let fx = (mouse_x as f64 / self.params.size.0 as f64 - 0.5) / self.params.zoom + self.params.center.re;
        let fy = (mouse_y as f64 / self.params.size.1 as f64 - 0.5) / self.params.zoom + self.params.center.im;
        
        self.params.zoom *= zoom_factor;
        self.params.center = Complex::new(
            fx - (mouse_x as f64 / self.params.size.0 as f64 - 0.5) / self.params.zoom,
            fy - (mouse_y as f64 / self.params.size.1 as f64 - 0.5) / self.params.zoom
        );

        let params = self.params.clone();
        self.active_fractal = match &self.active_fractal {
            ActiveFractal::Mandelbrot(_) => {
                ActiveFractal::Mandelbrot(FractalCalculator::<MandelbrotSet>::new_mandelbrot(params))
            },
            ActiveFractal::Julia(_) => {
                ActiveFractal::Julia(FractalCalculator::<JuliaSet>::new_julia(params, self.params.julia_c))
            },
            ActiveFractal::BurningShip(_) => {
                ActiveFractal::BurningShip(FractalCalculator::<BurningShip>::new_burning_ship(params))
            },
            ActiveFractal::Tricorn(_) => {
                ActiveFractal::Tricorn(FractalCalculator::<Tricorn>::new_tricorn(params))
            },
            ActiveFractal::Newton(_) => {
                ActiveFractal::Newton(FractalCalculator::<NewtonSet>::new_newton(params))
            },
        };
        
        self.need_update = true;
    }
} 