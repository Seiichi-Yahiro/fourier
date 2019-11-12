extern crate num;
extern crate rayon;
extern crate wasm_bindgen;

mod train;
use train::get_points;

use num::complex::Complex;
use rayon::prelude::*;
use wasm_bindgen::__rt::core::f32::consts::PI;
use wasm_bindgen::prelude::*;

type FType = f32;
type ComplexType = Complex<FType>;

const TWO_PI: FType = 2.0 * PI;

#[wasm_bindgen]
pub struct RotatingCircles {
    circles: Vec<RotatingCircle>,
}

#[wasm_bindgen]
impl RotatingCircles {
    pub fn new() -> RotatingCircles {
        RotatingCircles {
            circles: dtf(get_points())
                .into_iter()
                .enumerate()
                .map(complex_to_rotating_circle)
                .collect::<Vec<RotatingCircle>>(),
        }
    }

    pub fn create_points(&self, t: FType) -> Box<[FType]> {
        let mut points = Vec::<FType>::new();

        let mut x = 0.0;
        let mut y = 0.0;

        points.push(x);
        points.push(y);

        for circle in &self.circles {
            let phi = circle.frequency as FType * t * TWO_PI + circle.phase;
            x += circle.amplitude * phi.cos();
            y += circle.amplitude * phi.sin();

            points.push(x);
            points.push(y);
        }

        points.into_boxed_slice()
    }
}

struct RotatingCircle {
    pub amplitude: FType,
    pub phase: FType,
    pub frequency: usize,
}

fn dtf(x: Vec<ComplexType>) -> Vec<ComplexType> {
    let mut xks: Vec<ComplexType> = Vec::new();
    let big_n = x.len();

    for k in 0..big_n {
        let partial_phi = (TWO_PI * k as FType) / big_n as FType;

        let xk: ComplexType = x
            .iter()
            .enumerate()
            .map(|(n, x)| {
                let phi = partial_phi * n as FType;
                x * Complex::new(phi.cos(), -phi.sin())
            })
            .sum();

        xks.push(xk / big_n as FType);
    }

    xks
}

fn dtf_parallel(x: Vec<ComplexType>) -> Vec<ComplexType> {
    let big_n = x.len();

    let xks: Vec<ComplexType> = (0..big_n)
        .into_par_iter()
        .map(|k: usize| {
            let xk: ComplexType = x
                .iter()
                .enumerate()
                .map(|(n, x)| {
                    let phi = (TWO_PI * k as FType * n as FType) / big_n as FType;
                    x * Complex::new(phi.cos(), -phi.sin())
                })
                .sum();

            xk / big_n as FType
        })
        .collect();

    xks
}

fn complex_to_rotating_circle(params: (usize, ComplexType)) -> RotatingCircle {
    RotatingCircle {
        amplitude: params.1.norm(),
        phase: params.1.arg(),
        frequency: params.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtf_parallel() {
        let complex = dtf_parallel(get_points());
        println!("{}", complex[0]);
    }

    #[test]
    fn test_dtf() {
        let complex = dtf(get_points());
        println!("{}", complex[0]);
    }
}
