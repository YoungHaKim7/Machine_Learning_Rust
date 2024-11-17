use crate::print_type_of;

pub fn val() {
    let x = "learning rust";
    let y = 6;
    let z = 3.14;

    println!("{},", x);
    print!("type of val (x) : ");
    print_type_of(&x);
    println!("{},", y);
    print!("type of val (y) : ");
    print_type_of(&y);
    println!("{},", z);
    print!("type of val (z) : ");
    print_type_of(&z);
}
