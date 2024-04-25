fn main() {
    let s1 = String::from("hello");
    // let s1 = "hello";
    let s2 = s1;

    println!("{}, world!", s2);

    let mut s3 = String::from("hello,");
    s3.push_str(" world");
    println!("{}!", s3);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);

    let x = 5;
    let y = x;
    println!("x is {}, y is {}, x === y is {}", x, y, x == y);

    let x = "hello, world";
    let y = x;
    println!("x is {}, y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello world");
    takes_ownership(s);
    // not work
    // println!("s is {}", s);

    let x: u32 = 5;
    make_copy(x);
    println!("out x is {}", x);

    let s1 = give_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2.clone());
    println!("s1 is {}, s2 is {}, s3 is {}", s1, s2, s3);
}

fn takes_ownership(s: String) {
    println!("s is {}", s);
}

fn make_copy(x: u32) {
    println!("x is {}", x);
}

fn give_ownership() -> String {
    let s1 = String::from("hello");
    s1
}

fn takes_and_gives_back(str: String) -> String {
    str
}
