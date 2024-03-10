fn main() {
    println!("{}", fib(12));
}

fn fib(n: i32) -> i32 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
