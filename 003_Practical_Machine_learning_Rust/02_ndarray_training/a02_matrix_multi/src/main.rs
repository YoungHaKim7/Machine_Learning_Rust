use ndarray::array;

fn main() {
    let a = array![[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]];
    let b = array![[1], [3], [3], [3]];
    println!("{}", a);
    println!("{}", b);
    let multi_result = &a * &b;
    println!("a * b matrix = {}", multi_result);
}
