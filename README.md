# Fibonacci Algorithms in Rust

This repository contains implementations of the Fibonacci sequence in Rust using both recursive and iterative methods. Additionally, it includes a simple benchmark to compare the performance of these methods.

## Implementations

### Recursive Method

The recursive method is straightforward but can become inefficient for larger values of `n` due to repeated calculations.

```rust
fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
} ```


```rust
fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}```
###License
This project is licensed under the MIT License. See the LICENSE file for details.

Feel free to further customize this README as needed. If you have any more questions or need additional assistance, let me know! 😊

####Benchmarking
We use std::time::Instant to measure the execution time of both methods. This approach allows us to compare the performance directly.
use std::time::Instant;

fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}
```rust
fn main() {
    let n = 30;

    // Measure execution time for recursive method
    let start_recursive = Instant::now();
    let result_recursive = fibonacci_recursive(n);
    let duration_recursive = start_recursive.elapsed();
    println!("Fibonacci recursive({}) = {}, took {:?}", n, result_recursive, duration_recursive);

    // Measure execution time for iterative method
    let start_iterative = Instant::now();
    let result_iterative = fibonacci_iterative(n);
    let duration_iterative = start_iterative.elapsed();
    println!("Fibonacci iterative({}) = {}, took {:?}", n, result_iterative, duration_iterative);
} ```


### Results  

Fibonacci recursive(30) = 832040, took 8.538ms
Fibonacci iterative(30) = 832040, took 510ns
