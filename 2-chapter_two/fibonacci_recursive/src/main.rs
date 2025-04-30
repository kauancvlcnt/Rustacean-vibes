fn main() {
    for element in 1..50 {
        fib(element);
        println!("{}", fib(element));
    }
}

fn fib(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}
