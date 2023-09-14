use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secrect_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input a number!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("read input number fail");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("you guessed number is: {guess}");

        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
