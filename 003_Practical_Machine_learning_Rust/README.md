# Practical Machine Learning with Rust by Joydeep Bhattacharjee (Apress, 2020
- [This repository accompanies Practical Machine Learning with Rust by Joydeep Bhattacharjee (Apress, 2020).https://github.com/Apress/practical-machine-learning-w-rust](https://github.com/Apress/practical-machine-learning-w-rust)

# Rust array(`#[rustfmt::skip]`)

- https://users.rust-lang.org/t/guard-lines-from-rustfmt/102702/2

```rs
let s = [
    1, 2, 3, //
    4, 5, 6, //
    7, 8, 9, //
];


let matrix_2d = [
    [1, 2, 3], //
    [4, 5, 6], //
    [7, 8, 9], //
];

#[rustfmt::skip]
let matrix_2d = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];
#[rustfmt::skip]
let mat_mul = [
    [1],
    [2],
    [3]
];
```

- 2d matrix sample

```rs
fn main() {
    let matrix_2d = [
        [1, 2, 3], //
        [4, 5, 6], //
        [7, 8, 9], //
    ];
    for mat_2d in matrix_2d {
        println!("{:?}", mat_2d);
    }
    println!();
}
```

- Result
  
```bash

$ cargo r

[1, 2, 3]
[4, 5, 6]
[7, 8, 9]
```
