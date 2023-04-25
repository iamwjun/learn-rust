use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);


    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5.to_string());

    println!("s is {}, n is {}", s, n);


    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    println!("100's fibonacci is {}", fibonacci(7));
}

fn fibonacci(n: i32) -> i32 {
    let mut vec = Vec::new();
    for i in 0..(n + 1) {
        match i {
            0 | 1 => vec.push(i),
            _ => vec.push((vec[(i - 1) as usize] + vec[(i - 2) as usize]) % 1000000007)
        }
    }
    vec[n as usize]
} 