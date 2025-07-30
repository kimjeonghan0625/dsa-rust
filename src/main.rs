use std::{collections::HashMap, time::Instant};

// iterative way
fn fib1(num: i32) -> i32 {
    let (mut first, mut second, mut sum) = (1, 1, 0);
    (0..num - 2).into_iter().for_each(|_| {
        sum = first + second;
        first = second;
        second = sum;
    });
    sum
}

// recursive way
fn fib2(num: i32) -> i32 {
    fn inner_fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        inner_fib(n - 1) + inner_fib(n - 2)
    }
    inner_fib(num)
}

// recursive way with memoization
fn fib3(num: i32) -> i32 {
    let mut cache_vec = HashMap::<i32, i32>::new();

    fn inner_fib(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            if let None = cache.get(&n) {
                cache.insert(n, n);
            }
            return n;
        }
        let a = match cache.get(&(n - 1)) {
            Some(cached_val) => *cached_val,
            None => {
                let res = inner_fib(n - 1, cache);
                cache.insert(n - 1, res);
                res
            }
        };
        let b = match cache.get(&(n - 2)) {
            Some(cached_val) => *cached_val,
            None => {
                let res = inner_fib(n - 2, cache);
                cache.insert(n - 2, res);
                res
            }
        };
        a + b
    }

    inner_fib(num, &mut cache_vec)
}

fn main() {
    // iterative way
    let start = Instant::now();
    println!("fib1: {}", fib1(46));
    let duration = start.elapsed();
    println!("fib1 duration is {:?}", duration);

    // recursive way
    let start = Instant::now();
    println!("fib2: {}", fib2(46));
    let duration = start.elapsed();
    println!("fib2 duration is {:?}", duration);

    // recursive way with memoization
    let start = Instant::now();
    println!("fib3: {}", fib3(46));
    let duration = start.elapsed();
    println!("fib3 duration is {:?}", duration);
}
