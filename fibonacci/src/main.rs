fn fib(a:u64, b:u64, i:u64) -> u64 {
    if i <= 0 { return a; }
    return fib(b, a+b, i-1);
}

fn main() {
    for i in 0..93 {
        println!("{}: {}", i, fib(0, 1, i));
    }
}
