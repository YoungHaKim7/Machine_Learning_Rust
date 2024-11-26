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

#[inline(always)]
pub fn fast_sigmoid(v: f32) -> f32 {
    const C1: f32 = 0.03138777;
    const C2: f32 = 0.276281267;
    const C_LOG2F: f32 = 1.442695022; // ln(2) reciprocal

    let mut v = v * C_LOG2F;
    let int_part = v as i32; // Extract integer part
    let x = v - int_part as f32; // Get fractional part
    let xx = x * x; // Square of the fractional part
    let v1 = C_LOG2F + C2 * xx;
    let v2 = x + xx * C1 * x;
    let mut v3 = v2 + v1;

    // Combine integer part into the exponent
    let bits = (v3.to_bits() as i32) + (int_part << 23);
    v3 = f32::from_bits(bits as u32);

    let v4 = v2 - v1;
    let res = v3 / (v3 - v4); // For tanh: change to (v3 + v4) / (v3 - v4)
    res
}

fn main() {
    benchmark("atan(pi*x/2)*2/pi", with_atan);
    benchmark("atan(x)", |x| x.atan());
    benchmark("1/(1+exp(-x))", with_exp);
    benchmark("1/sqrt(1+x^2)", with_sqrt);
    benchmark("erf(sqrt(pi)*x/2)", with_erf);
    benchmark("tanh(x)", |x| x.tanh());
    benchmark("x/(1+|x|)", with_fabs);
    let value = 1.0f32; // Example input
    let result = fast_sigmoid(value);
    println!("FastSigmoid({}): {}", value, result);
}
