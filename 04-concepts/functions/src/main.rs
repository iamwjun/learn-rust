fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");

    another_function();

    print_number(8);

    print_labeled_measurement(6, 'z');

    print_rement();

    let x = five();
    print!("function five return: {x}\n");

    let y: i32 = plus_one(9);
    print!("function plus_one return: {y}\n");
}

fn another_function () {
    println!("Another function!");
}

fn print_number (x: i32) {
    println!("this number is {x}!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn print_rement() {
    let y = {
        let x = 5;
        x + 1
    };
    
    println!("this rement is {y}");
}
