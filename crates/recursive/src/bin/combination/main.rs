use recursive::combination_util::{c, c_with_pascal_triangle};

fn main() {
    let five_c_two = c(4, 2);
    println!("first method: {five_c_two}");
    let five_c_two = c_with_pascal_triangle(4, 2);
    println!("second method: {five_c_two}");
}
