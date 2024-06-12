use std::fmt::Debug;
use std::fmt::Display;
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn main() {
    let author: &str = "Machael Jordan";
    print_author(author);

    print(author);

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );

    let i = 5;

    print_it(&i);
}

fn print_it<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print<T: Display + 'static>(message: T) {
    println!("{}", message);
}

fn print_author(author: &'static str) {
    println!("{}", author);
}

fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}
