pub fn c(n: i32, r: i32) -> i32 {
    fn factorial(n: i32) -> i32 {
        (1..=n).product::<i32>()
    }
    let n_fac = factorial(n);
    let r_fac = factorial(r);
    let n_minus_r_fac = factorial(n - r);
    (n_fac / r_fac) / n_minus_r_fac
}

pub fn c_with_pascal_triangle(n: i32, r: i32) -> i32 {
    if r == 0 || r == n {
        return 1;
    }
    c_with_pascal_triangle(n-1, r-1) + c_with_pascal_triangle(n-1, r)
}