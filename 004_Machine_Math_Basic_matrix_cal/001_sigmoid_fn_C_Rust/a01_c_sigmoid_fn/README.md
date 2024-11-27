# Result

```bash
atan(pi*x/2)*2/pi    9.8 ns
atan(x)              7.5 ns
1/(1+exp(-x))        7.3 ns
1/sqrt(1+x^2)        4.0 ns
erf(sqrt(pi)*x/2)    4.0 ns
tanh(x)              2.0 ns
x/(1+|x|)            2.3 ns
```

- `clang -Wall -O2 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb -o ./target/main ./src/main.c`

```bash
clang -Wall -O2 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb -o ./target/main ./src/main.c
./src/main.c:56:14: warning: unused parameter 'argc' [-Wunused-parameter]
int main(int argc, char **argv) {
             ^
./src/main.c:56:27: warning: unused parameter 'argv' [-Wunused-parameter]
int main(int argc, char **argv) {
                          ^
2 warnings generated.
./target/main
atan(pi*x/2)*2/pi    6.8 ns
atan(x)              6.4 ns
1/(1+exp(-x))        8.4 ns
1/sqrt(1+x^2)        0.0 ns
erf(sqrt(pi)*x/2)    2.7 ns
tanh(x)              2.2 ns
x/(1+|x|)            0.0 ns
```
