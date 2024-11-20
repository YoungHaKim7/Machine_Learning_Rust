use ndarray::{Array, ShapeBuilder};

fn main() {
    let a = Array::<u64, _>::zeros((2, 3, 4).f());
    println!("{:?}", a);
}
