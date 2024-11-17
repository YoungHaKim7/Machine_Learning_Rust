#![feature(core_intrinsics)]

use variables1::val;

mod variables1;

fn print_type_of<T>(_: &T) {
    println!("{}", std::intrinsics::type_name::<T>());
}
fn main() {
    let x = "learning rust";
    println!("{}", x);
    print_type_of(&x);
    val();
}
