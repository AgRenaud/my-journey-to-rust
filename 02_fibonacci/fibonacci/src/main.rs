use std::io;

fn main() {
    println!("How many steps of the Fibonacci's suite you want ?");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("invalid input");

    println!("result: {}", fib(n, 0, 1));
}

fn fib(n: u32, a: u32, b: u32) -> u32 {
    if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    } else {
        return fib(n - 1, b, a + b);
    }
}
