use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input a number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("read input number fail");

    println!("you guessed number is: {guess}");
}
