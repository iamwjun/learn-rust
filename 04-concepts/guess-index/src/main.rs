use std::io;

fn main() {
    println!("Please guess the index!");

    let arr = [1, 2, 3, 4, 5, 6];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("please enter a number");

    let index: usize = index
        .trim()
        .parse()
        .expect("please enter a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
