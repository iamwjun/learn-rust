struct Struct {
    e: i32
}

fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);

    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let guess = "42".parse::<i32>().expect("Not a number!");

    println!("guess is {}", guess);

    let m1 = 98_222;
    let m2 = 0x12;
    let m3 = 0o10;
    let m4 = 0b10000;

    println!("m is {} {} {} {}", m1, m2, m3, m4);
}
