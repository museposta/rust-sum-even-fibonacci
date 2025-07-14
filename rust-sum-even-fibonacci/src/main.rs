use std::io::{self, BufRead};

fn sum_even_fibonacci(n: i64) -> i64 {
    let mut sum = 0;
    let mut a = 2;
    let mut b = 8;
    if n >= 2 {
        sum += a;
    }
    if n >= 8 {
        sum += b;
    }
    let mut c = 4 * b + a;
    while c <= n {
        sum += c;
        a = b;
        b = c;
        c = 4 * b + a;
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();
        println!("{}", sum_even_fibonacci(n));
    }
}
