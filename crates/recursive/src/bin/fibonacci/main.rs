use std::time::Instant;

use recursive::fibonacci_util::{fib1, fib2, fib3};

fn main() {
    // iterative way
    let start = Instant::now();
    println!("fib1: {}", fib1(46));
    let duration = start.elapsed();
    println!("fib1 duration is {duration:?}");

    // recursive way
    let start = Instant::now();
    println!("fib2: {}", fib2(46));
    let duration = start.elapsed();
    println!("fib2 duration is {duration:?}");

    // recursive way with memoization
    let start = Instant::now();
    println!("fib3: {}", fib3(46));
    let duration = start.elapsed();
    println!("fib3 duration is {duration:?}");
}
