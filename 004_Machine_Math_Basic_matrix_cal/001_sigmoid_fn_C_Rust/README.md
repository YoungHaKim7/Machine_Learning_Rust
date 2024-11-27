- https://stackoverflow.com/questions/10732027/fast-sigmoid-algorithm

<hr />


# Fast sigmoid algorithm 공식_출처 : stackoverflow.com
- https://stackoverflow.com/questions/10732027/fast-sigmoid-algorithm

```math

S(t) = \frac{1}{1+e^{-t}} 

```

- sigmoid 그래프로 이해하기
  - https://stackoverflow.com/questions/2480650/what-is-the-role-of-the-bias-in-neural-networks

<img src="https://i.sstatic.net/ddyfr.png" />

- `S(t) = 1 / (1 + e^(-t))` (where `^` is `pow`)

I found that using the C built-in function `exp()` to calculate the value of `f(x)` is slow. Is there any faster algorithm to calculate the value of `f(x)`?

- 공식을 그래프로 보자 demos랑 다른거
  - demos로 만들어봄 https://www.desmos.com/calculator/zdoveye7qb
  - https://www.wolframalpha.com/input?i=+series+1+%2F+%281+%2B+e+%5E+%28-x%29%29+at+x%3D1

- C언어로
  - https://gist.github.com/astanin/5270668
