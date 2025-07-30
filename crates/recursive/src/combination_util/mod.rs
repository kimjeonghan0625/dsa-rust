pub fn c(n: i32, r: i32) -> i32 {
    fn factorial(n: i32) -> i32 {
        (1..=n).product::<i32>()
    }
    let n_fac = factorial(n);
    let r_fac = factorial(r);
    let n_minus_r_fac = factorial(n - r);
    (n_fac / r_fac) / n_minus_r_fac
}
