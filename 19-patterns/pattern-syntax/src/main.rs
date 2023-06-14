struct Point {
    x: i32,
    y: i32,
}

struct ThreeDPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color)
}

enum SeMessage {
    Hello { id: i32 },
}

fn main() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let msg = Message::ChangeColor(Color::Hsv(0, 189, 89));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Hsv(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        },
        _ => (),
    }
    
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("three"),
        3 => println!("four"),
        _ => print!("anything")
    }

    let a = None;
    let b = 10;

    match a {
        Some(5) => println!("Got 50"),
        Some(b) => println!("Matched, y = {b}"),
        _ => println!("Default case, x = {:?}", a),
    }

    println!("at the end: a = {:?}, b = {b}", a);

    let c = 1;

    match c {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything")
    }

    let m = 3;

    match m {
        1..=5 => println!("one through five"),
        _ => println!("something eles")
    }

    let n = 'a';

    match n {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: -1, y: 10 };
    let Point { x: a, y: b } = p;
    assert_eq!(-1, a);
    assert_eq!(10, b);

    let Point { x, y } = p;
    assert_eq!(-1, x);
    assert_eq!(10, y);

    let v = Point { x: 0, y: 7 };

    match v {
        Point { x: 0, y: 7 } => println!("axis: ({x}, {y})"),
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    bar(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}")
    }

    let s = Some(String::from("Hello!"));

    // error
    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    println!("{:?}", s);

    let origin = ThreeDPoint { x: 0, y: 0, z: 0 };

    match origin {
        ThreeDPoint { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    match numbers {
        (.., last) => {
            println!("Some numbers: {}", last)
        },
    }

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(50);
    let y = 50;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 8;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = SeMessage::Hello { id: 10 };

    match msg {
        SeMessage::Hello { id: id_value @ 3..=7 } => println!("Found an id in range: {}", id_value),
        SeMessage::Hello { id: id_value @ 10..=12 } => {
            println!("Found an id in another range {}", id_value)
        },
        SeMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn bar(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y)
}