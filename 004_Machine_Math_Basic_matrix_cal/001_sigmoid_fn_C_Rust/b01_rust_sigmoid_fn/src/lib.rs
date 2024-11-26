#![feature(test)]

extern crate test;

use rand::Rng;
use std::time::Instant;

const SIZE: usize = 100;
const CYCLES: usize = 10_000_000;
const M_PI_2: f64 = std::f64::consts::PI / 2.0;
const M_PI_2_INV: f64 = 1.0 / M_PI_2;
const M_2_SQRTPI: f64 = 1.12837916709551257390; // 2/sqrt(pi)
const ERF_COEF: f64 = 1.0 / M_2_SQRTPI;

fn with_atan(x: f64) -> f64 {
    M_PI_2_INV * (M_PI_2 * x).atan()
}

fn with_exp(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn with_sqrt(x: f64) -> f64 {
    1.0 / (1.0 + x * x).sqrt()
}

fn with_erf(x: f64) -> f64 {
    libm::erf(ERF_COEF * x)
}

fn with_fabs(x: f64) -> f64 {
    x / (1.0 + x.abs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn benchmark_atan(b: &mut Bencher) {
        b.iter(|| with_atan(1_000.0));
    }

    #[bench]
    fn benchmark_exp(b: &mut Bencher) {
        b.iter(|| with_exp(1_000.0));
    }

    #[bench]
    fn benchmark_sqrt(b: &mut Bencher) {
        b.iter(|| with_sqrt(1_000.0));
    }

    #[bench]
    fn benchmark_erf(b: &mut Bencher) {
        b.iter(|| with_erf(1_000.0));
    }

    fn benchmark_fabs(b: &mut Bencher) {
        b.iter(|| with_fabs(1_000.0));
    }
}
