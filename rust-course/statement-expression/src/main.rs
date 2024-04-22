fn main() {
    println!("x + y is {}", add_with_extra(3, 5));

    let _a = 8;
    let _b: Vec<f64> = Vec::new();
    let (_a, _c) = ("hi", false);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    assert_eq!(ret_unit_type(), ());
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 5;
    x + y
}

fn ret_unit_type() {
    let x = 1;
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    let z = if x % 2 == 1 { "odd" } else { "even" };
}
