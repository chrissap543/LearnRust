fn main() {
    println!("{}", fib(45));
}

fn fib(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return n
    }
    fib(n-1) + fib(n-2)
}