use rand::Rng;

#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("penny's value is {}", value_in_coin(Coin::Penny));

    println!(
        "quarter's value is {}",
        value_in_coin(Coin::Quarter(UsState::Alabama))
    );

    let five = Some(5);

    println!("five plus one is {:#?}", plus_one(five));

    println!("none plus one is {:#?}", plus_one(None));

    let mut rng = rand::thread_rng();
    let dice_roll = rng.gen_range(1..9);

    println!("current dice number is {}", dice_roll);

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
        // other => move_player(other)
    }
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn add_fancy_hat() {
    println!("you got a fancy hat! 🎇");
}

fn remove_fancy_hat() {
    println!("you lost a fancy hat! 😭");
}

fn move_player(step: u32) {
    println!("move {} steps! 😊", step);
}

fn reroll() {
    println!("please reroll!");
}
