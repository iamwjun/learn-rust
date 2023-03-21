use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        print!("Please input your guess number\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read your number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed: {guess}\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎆 You win!");
                break;
            }
        }
    }
}
