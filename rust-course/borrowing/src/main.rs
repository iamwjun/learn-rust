fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    println!("s change after is {:?}", change(&mut s));

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    println!("{}", r2);

    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing is {}", reference_to_nothing);
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change(str: &mut String) -> &mut String {
    str.push_str(", world");
    str
}

fn dangle() -> String {
    String::from("hello")
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
