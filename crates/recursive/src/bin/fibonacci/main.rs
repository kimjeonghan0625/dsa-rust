use std::time::Instant;

use recursive::fibonacci_util::{fib_iter, fib_recur, fib_recur_with_memoization};

fn main() {
    // iterative way
    let start = Instant::now();
    println!("fib1: {}", fib_iter(46));
    let duration = start.elapsed();
    println!("fib1 duration is {duration:?}");

    // recursive way
    let start = Instant::now();
    println!("fib2: {}", fib_recur(46));
    let duration = start.elapsed();
    println!("fib2 duration is {duration:?}");

    // recursive way with memoization
    let start = Instant::now();
    println!("fib3: {}", fib_recur_with_memoization(46));
    let duration = start.elapsed();
    println!("fib3 duration is {duration:?}");
}
