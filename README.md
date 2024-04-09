# Euclidean algorithm

There are two ways of implementing the euclidean algorithm.
In the first a very simple function is called recursivly.
The second is a function that is looping and manipulating the integer.

The main.rs is providing a binary, to test the two approaches and compare their runtime.

The function can be used by adding the dependency to the Cargo.toml and importing the function.

```
use crate::euclidean_algo::{eucl_algo_recursive, eucl_algo_loop};

// As with Rust adding integer to a function we are provided by a copy and don't need to make the initial integer to mutable them to mutable.
let x: u64 = 15;
let y: u64 = 30;

let gcd_recursive = eucl_algo_recursive::run(x,y);
let gcd_loop = eucl_algo_loop::run(x,y);

println!("The gcd is {gcd_recursive} (loop: {gcd_loop})");

```

For bigger integers a run_big function is provided.
You can call `::run_bigint(x,y)` instead of run, using `num::BigUint` to process bigger numbers.
