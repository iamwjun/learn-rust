use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_world() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}