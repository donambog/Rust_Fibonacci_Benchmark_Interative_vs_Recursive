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
}
