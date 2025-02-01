use num::Complex;

pub trait FractalFunction: Clone {
    fn iterate(&self, c: Complex<f64>, z: Complex<f64>, max_iter: u32) -> (u32, Complex<f64>);
    fn initial_z(&self) -> Complex<f64> {
        Complex::new(0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct MandelbrotSet;

#[derive(Clone)]
pub struct JuliaSet {
    pub c: Complex<f64>,
}

#[derive(Clone)]
pub struct BurningShip;

#[derive(Clone)]
pub struct Tricorn;

#[derive(Clone)]
pub struct NewtonSet;

impl FractalFunction for MandelbrotSet {
    fn iterate(&self, c: Complex<f64>, z: Complex<f64>, max_iter: u32) -> (u32, Complex<f64>) {
        let mut z = z;
        let mut i = 0;
        
        while i < max_iter && z.norm_sqr() <= 4.0 {
            z = z * z + c;
            i += 1;
        }
        (i, z)
    }
}

impl FractalFunction for JuliaSet {
    fn iterate(&self, _c: Complex<f64>, z: Complex<f64>, max_iter: u32) -> (u32, Complex<f64>) {
        let mut z = z;
        let mut i = 0;
        
        while i < max_iter && z.norm_sqr() <= 4.0 {
            z = z * z + self.c;
            i += 1;
        }
        (i, z)
    }
}

impl FractalFunction for BurningShip {
    fn iterate(&self, c: Complex<f64>, z: Complex<f64>, max_iter: u32) -> (u32, Complex<f64>) {
        let mut z = z;
        let mut i = 0;
        
        while i < max_iter && z.norm_sqr() <= 4.0 {
            let re = f64::abs(z.re);
            let im = f64::abs(z.im);
            z = Complex::new(re, im) * Complex::new(re, im) + c;
            i += 1;
        }
        (i, z)
    }
}

impl FractalFunction for Tricorn {
    fn iterate(&self, c: Complex<f64>, z: Complex<f64>, max_iter: u32) -> (u32, Complex<f64>) {
        let mut z = z;
        let mut i = 0;
        
        while i < max_iter && z.norm_sqr() <= 4.0 {
            z = Complex::new(z.re, -z.im) * Complex::new(z.re, -z.im) + c;
            i += 1;
        }
        (i, z)
    }
}

impl FractalFunction for NewtonSet {
    fn iterate(&self, _c: Complex<f64>, z: Complex<f64>, max_iter: u32) -> (u32, Complex<f64>) {
        let mut z = z;
        let mut i = 0;
        let tolerance = 1e-6;
        
        while i < max_iter {
            let z2 = z * z;
            let z3 = z2 * z;
            
            let root1 = Complex::new(1.0, 0.0);
            let root2 = Complex::new(-0.5, 0.866);
            let root3 = Complex::new(-0.5, -0.866);
            
            if (z - root1).norm() < tolerance || 
               (z - root2).norm() < tolerance || 
               (z - root3).norm() < tolerance {
                break;
            }
            
            z = z - (z3 - Complex::new(1.0, 0.0)) / (Complex::new(3.0, 0.0) * z2);
            i += 1;
        }
        (i, z)
    }
}

// ... autres implémentations similaires ... 