use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("error");

    let number = number.trim().parse().expect("error");
    let fib = fib(number);

    println!("the {number}th fibonacci number is {fib}");

    let fib: usize = fibonacci(number);

    println!("the {number}th fibonacci number is {fib}");

    let fib = fib2(50);

    println!("the 50th fibonacci number is {fib}");
}

fn fib2(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let tmp = b;
        b = (a + b) % 1000000007; // 对结果进行取模操作
        a = tmp;
    }
    return b;
}

fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return  n;
    }

    return fibonacci(n - 1) + fibonacci(n -2);
}

fn fib(num: usize) -> usize {
    if num < 2 {
        return  num;
    }

    let mut a: usize;
    let mut b: usize = 0;
    let mut c: usize = 1;
    let mut d: usize = 2;

    while d <= num {
        a = b;
        b = c;
        c = a + b;
        d += 1;
    }

    return  c;
}
