use std::io;
use std::time::Instant;
use num::*;
use euclidean_algo::{eucl_algo_recursive, eucl_algo_loop};

fn main() {
    let mut x: BigUint = request_param("Enter number: ");
    let mut y: BigUint = request_param("Enter number: ");
    if x < y {
        (x, y) = (y, x);
    }
    println!("Start...");
    let sr = Instant::now();
    let gcd: BigUint = eucl_algo_recursive::run_bigint(&x, &y);
    let er = Instant::now();
    let rt = er.duration_since(sr).as_secs_f64();
    println!("Result of {x} and {y} is {gcd}!");
    println!("Recursion took {rt} seconds.");
    let sl = Instant::now();
    let gcd2: BigUint = eucl_algo_loop::run_bigint(x.clone(), y.clone());
    let el = Instant::now();
    let lt = el.duration_since(sl).as_secs_f64();
    println!("Result of {x} and {y} is {gcd2}!");
    println!("Looping took {lt} seconds");
}

fn request_param(text: &str) -> BigUint {
    let mut input_line = String::new();
    println!("{text}");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Unable reading input.");
    return input_line
        .trim()
        .parse()
        .expect("Input is not valid.");
}

