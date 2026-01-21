use fractal_generator::{
    fractal_calculator::{FractalCalculator, QualityLevel},
    fractal_params::FractalParams,
    fractal_types::{MandelbrotSet, JuliaSet, BurningShip, NewtonSet},
    color_schemes::{FireScheme, OceanScheme, RainbowScheme, ClassicScheme, GrayscaleScheme},
};
use num::Complex;

fn main() {
    let width = 1200;
    let height = 900;
    let size = (width, height);
    
    println!("Generating Mandelbrot (Fire)...");
    let mut params = FractalParams::default();
    params.size = size;
    params.zoom = 1.0;
    params.center = Complex::new(-0.7, 0.0);
    params.max_iterations = 200;
    
    let calc = FractalCalculator::<MandelbrotSet>::new_mandelbrot(params.clone());
    let buffer = calc.generate(&FireScheme);
    save_image("mandelbrot_fire.png", width as u32, height as u32, &buffer);

    println!("Generating Julia (Ocean)...");
    let mut params_julia = params.clone();
    params_julia.zoom = 1.2;
    params_julia.julia_c = Complex::new(-0.7, 0.27015);
    params_julia.center = Complex::new(0.0, 0.0);
    let calc = FractalCalculator::<JuliaSet>::new_julia(params_julia, Complex::new(-0.7, 0.27015));
    let buffer = calc.generate(&OceanScheme);
    save_image("julia_ocean.png", width as u32, height as u32, &buffer);

    println!("Generating Burning Ship (Rainbow)...");
    let mut params_ship = params.clone();
    params_ship.zoom = 1.8;
    params_ship.center = Complex::new(-1.75, -0.04);
    params_ship.max_iterations = 300;
    let calc = FractalCalculator::<BurningShip>::new_burning_ship(params_ship);
    let buffer = calc.generate(&RainbowScheme);
    save_image("burning_ship_rainbow.png", width as u32, height as u32, &buffer);

    println!("Generating Newton (Classic)...");
    let mut params_newton = params.clone();
    params_newton.zoom = 1.0;
    params_newton.center = Complex::new(0.0, 0.0);
    params_newton.max_iterations = 50;
    let calc = FractalCalculator::<NewtonSet>::new_newton(params_newton);
    let buffer = calc.generate(&ClassicScheme);
    save_image("newton_classic.png", width as u32, height as u32, &buffer);
    
    println!("Done!");
}

fn save_image(filename: &str, width: u32, height: u32, data: &[u8]) {
    image::save_buffer(
        filename,
        data,
        width,
        height,
        image::ColorType::Rgba8
    ).expect("Failed to save image");
}
