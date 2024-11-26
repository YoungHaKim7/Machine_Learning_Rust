use rand::Rng;
use std::time::Instant;

const SIZE: usize = 100;
const CYCLES: usize = 10_000_000;
const M_PI_2: f64 = std::f64::consts::PI / 2.0;
const M_PI_2_INV: f64 = 1.0 / M_PI_2;
const M_2_SQRTPI: f64 = 1.12837916709551257390; // 2/sqrt(pi)
const ERF_COEF: f64 = 1.0 / M_2_SQRTPI;

fn benchmark<F>(name: &str, fun: F)
where
    F: Fn(f64) -> f64,
{
    let mut rng = rand::thread_rng();
    let xs: Vec<f64> = (0..SIZE).map(|_| rng.gen::<f64>()).collect();

    let start = Instant::now();
    for _ in 0..CYCLES {
        for &x in &xs {
            let _ = fun(x);
        }
    }
    let duration = start.elapsed();
    let t_ns = (duration.as_secs_f64() * 1e9) / (CYCLES as f64 * SIZE as f64);
    println!("{:<17} {:6.1} ns", name, t_ns);
}

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

fn main() {
    benchmark("atan(pi*x/2)*2/pi", with_atan);
    benchmark("atan(x)", |x| x.atan());
    benchmark("1/(1+exp(-x))", with_exp);
    benchmark("1/sqrt(1+x^2)", with_sqrt);
    benchmark("erf(sqrt(pi)*x/2)", with_erf);
    benchmark("tanh(x)", |x| x.tanh());
    benchmark("x/(1+|x|)", with_fabs);
}
