use std::io;

fn main() {
    println!("Enter a position in the fibonacci sequence");

    let mut limit = String::new();

    io::stdin()
        .read_line(&mut limit)
        .expect("Failed to read line");

    let limit: u32 = limit.trim().parse().expect("Enter a whole number");
    let result = fib(limit, 0, 1);

    // This does print 1th and other incorrect numbers, will fix later
    println!("The {}th fibonacci number is {}", limit, result);
}

fn fib(limit: u32, n1: u32, n2: u32) -> u32{
    // The first two fibonacci numbers are already entered. Recursion stops at n - 2 iterations.
    if limit > 2 {
        return fib(limit - 1, n2, n1 + n2)
    }
    else {
        return n2;
    }
}
