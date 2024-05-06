#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("dire is east"),
        Direction::North | Direction::South => println!("dire is north or south"),
        _ => println!("dire is west"),
    }

    let dire = Direction::West;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };

    println!("coin is {}", value_in_cents(Coin::Penny));

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => println!("{}", s),
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }
    match v {
        Some(3) =>    println!("three"),
        _ => (),
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    let foo = 'f';
    dbg!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    dbg!(matches!(bar, Some(x) if x > 2));

    let age = Some(30);
    println!("befor match, age is{:?}", age);
    if let Some(age) = age {
        println!("match age is {}", age);
    }

    println!("after match, age is {:?}", age);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
