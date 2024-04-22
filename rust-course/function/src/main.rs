use std::fmt::Debug;

fn main() {
    println!("i + j is {}", add(3, 5));

    let a: Vec<i32> = [1, 2, 3, 4].to_vec();
    report(a);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn report<T: Debug>(item: T) {
    println!("item is {:?}", item);
}