# Result

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


```

