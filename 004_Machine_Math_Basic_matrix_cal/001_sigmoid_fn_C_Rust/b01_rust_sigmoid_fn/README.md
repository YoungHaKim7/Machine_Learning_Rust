# Result

- https://github.com/chhetripradeep/cargo-bench-example

- `cargo r --release` 최적화 이빠이

```bash
$ cargo r --release
    Finished `release` profile [optimized] target(s) in 15.90s

atan(pi*x/2)*2/pi    6.7 ns
atan(x)              5.9 ns
1/(1+exp(-x))        0.0 ns
1/sqrt(1+x^2)        0.0 ns
erf(sqrt(pi)*x/2)    4.5 ns
tanh(x)             14.8 ns
x/(1+|x|)            0.0 ns

FastSigmoid(1): 0.7869864

```


- cargo bench

```bash
running 6 tests
test tests::benchmark_atan         ... bench:           0.24 ns/iter (+/- 0.00)
test tests::benchmark_erf          ... bench:           2.16 ns/iter (+/- 0.03)
test tests::benchmark_exp          ... bench:           0.24 ns/iter (+/- 0.00)
test tests::benchmark_fabs         ... bench:           0.24 ns/iter (+/- 0.00)
test tests::benchmark_fast_sigmoid ... bench:           0.24 ns/iter (+/- 0.00)
test tests::benchmark_sqrt         ... bench:           0.24 ns/iter (+/- 0.00

```
