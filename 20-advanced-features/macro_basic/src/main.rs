use macro_basic::{vec, HelloMacro};

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    println!("v is {:?}", v);

    Pancakes::hello_macro();
}