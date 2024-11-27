use b01_rust_sigmoid_fn::{
    benchmark, fast_sigmoid, with_atan, with_erf, with_exp, with_fabs, with_sqrt,
};

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
